//! Dioxus Plugin for Bevy
#![allow(non_snake_case)]

use crate::{
    context::UiContext,
    converter,
    event::{KeyboardEvent, UiEvent, VDomCommand, WindowEvent},
    runner::runner,
    setting::DioxusSettings,
    stage::UiStage,
    window::DioxusWindows,
};

use bevy::{
    app::prelude::*,
    ecs::{event::Events, prelude::*},
    input::InputPlugin,
    log::error,
    log::warn,
    math::{UVec2, Vec2},
    window::{
        CreateWindow, ModifiesWindows, WindowClosed, WindowCommand, WindowCreated, WindowMode,
        WindowPlugin, WindowScaleFactorChanged, Windows,
    },
};
use bevy_dioxus_core::prelude::GlobalStateHandler;
use dioxus_core::{Component as DioxusComponent, SchedulerMsg, VirtualDom};
use fermi::AtomRoot;
use futures_channel::mpsc;
use futures_intrusive::channel::{
    shared::{channel, Receiver, Sender},
    TrySendError,
};
use std::{
    fmt::Debug,
    marker::PhantomData,
    rc::Rc,
    sync::{Arc, Mutex},
};
use tokio::{runtime::Runtime, select};
use wry::application::{
    dpi::{LogicalPosition, LogicalSize, PhysicalPosition},
    event_loop::EventLoop,
    window::Fullscreen,
};

/// Dioxus Plugin for Bevy
pub struct DioxusPlugin<GlobalState, CoreCommand, UiCommand, Props = ()> {
    /// Root component
    pub Root: DioxusComponent<Props>,

    global_state_type: PhantomData<GlobalState>,
    core_cmd_type: PhantomData<CoreCommand>,
    ui_cmd_type: PhantomData<UiCommand>,
}

impl<GlobalState, CoreCommand, UiCommand, Props> Plugin
    for DioxusPlugin<GlobalState, CoreCommand, UiCommand, Props>
where
    GlobalState: 'static + Send + Sync + GlobalStateHandler,
    CoreCommand: 'static + Send + Sync + Clone + Debug,
    UiCommand: 'static + Send + Sync + Clone + Debug,
    Props: 'static + Send + Sync + Clone + Default,
{
    fn build(&self, app: &mut App) {
        let (core_tx, core_rx) = channel::<CoreCommand>(8);
        let (ui_tx, ui_rx) = channel::<UiCommand>(8);
        let (vdom_cmd_tx, vdom_cmd_rx) = channel::<VDomCommand<GlobalState>>(8);

        let settings = app
            .world
            .remove_non_send_resource::<DioxusSettings<Props>>()
            .unwrap_or_default();

        app.add_plugin(WindowPlugin::default())
            .add_plugin(UiStagePlugin)
            .add_plugin(InputPlugin)
            .add_event::<KeyboardEvent>()
            .add_event::<CoreCommand>()
            .add_event::<UiCommand>()
            .insert_resource(core_rx)
            .insert_resource(ui_tx)
            .insert_resource(Runtime::new().unwrap())
            .insert_resource(vdom_cmd_tx)
            .insert_non_send_resource(settings)
            .init_non_send_resource::<DioxusWindows>()
            .insert_non_send_resource(EventLoop::<UiEvent<CoreCommand>>::with_user_event())
            .set_runner(|app| runner::<CoreCommand, UiCommand, Props>(app))
            .add_system_to_stage(UiStage::Update, send_ui_commands::<UiCommand>)
            .add_system_to_stage(UiStage::Update, change_window.label(ModifiesWindows));

        self.spawn_virtual_dom(&mut app.world, (core_tx, ui_rx), vdom_cmd_rx);
        Self::handle_initial_window_events(&mut app.world);
    }
}

impl<GlobalState, CoreCommand, UiCommand, Props>
    DioxusPlugin<GlobalState, CoreCommand, UiCommand, Props>
where
    GlobalState: Send + Sync + GlobalStateHandler,
    CoreCommand: Clone + Debug + Send + Sync,
    UiCommand: Clone + Debug + Send + Sync,
    Props: Send + Sync + Clone + 'static,
{
    /// Initialize DioxusPlugin with root component and channel types
    ///
    /// ```no_run
    /// use bevy::prelude::*;
    /// use bevy_dioxus::desktop::prelude::*;
    /// use dioxus::prelude::*;
    ///
    /// // DioxusPlugin accepts any types as command. Pass empty tuple if channel is not necessary.
    /// type CoreCommand = ();
    /// type UiCommand = ();
    ///
    /// fn main() {
    ///    App::new()
    ///         .add_plugin(DioxusPlugin::<EmptyGlobalState, CoreCommand, UiCommand>::new(Root))
    ///         .run();
    /// }
    ///
    /// fn Root(cx: Scope) -> Element {
    ///    cx.render(rsx! {
    ///    h1 { "Hello, World !" }
    ///        })
    /// }
    /// ```
    pub fn new(Root: DioxusComponent<Props>) -> Self {
        Self {
            Root,
            core_cmd_type: PhantomData,
            ui_cmd_type: PhantomData,
            global_state_type: PhantomData,
        }
    }

    fn spawn_virtual_dom(
        &self,
        world: &mut World,
        (core_tx, ui_rx): (Sender<CoreCommand>, Receiver<UiCommand>),
        vdom_cmd_rx: Receiver<VDomCommand<GlobalState>>,
    ) {
        let (dom_tx, dom_rx) = mpsc::unbounded::<SchedulerMsg>();
        let edit_queue = Arc::new(Mutex::new(Vec::new()));
        let settings = world
            .get_non_send_resource::<DioxusSettings<Props>>()
            .unwrap();
        let Root = self.Root.clone();
        let props = settings.props.as_ref().unwrap().clone();
        let event_loop = world
            .get_non_send_resource::<EventLoop<UiEvent<CoreCommand>>>()
            .unwrap();
        let proxy = event_loop.create_proxy();
        let context = UiContext::<CoreCommand, UiCommand>::new(proxy.clone(), (core_tx, ui_rx));

        world.insert_resource(dom_tx.clone());
        world.insert_resource(edit_queue.clone());

        std::thread::spawn(move || {
            // initialize vdom
            let mut vdom = VirtualDom::new_with_props_and_scheduler(Root, props, (dom_tx, dom_rx));

            // set UI context
            vdom.base_scope().provide_context(context.clone());

            // apply initial edit
            let initial_muts = vdom.rebuild();
            edit_queue
                .lock()
                .unwrap()
                .push(serde_json::to_string(&initial_muts.edits).unwrap());
            proxy
                .send_event(UiEvent::WindowEvent(WindowEvent::Update))
                .unwrap();

            let cx = vdom.base_scope();
            let root = match cx.consume_context::<Rc<AtomRoot>>() {
                Some(root) => root,
                None => cx.provide_root_context(Rc::new(AtomRoot::new(cx.schedule_update_any()))),
            };

            Runtime::new().unwrap().block_on(async move {
                loop {
                    // wait for either
                    select! {
                        () = vdom.wait_for_work() => {} // 1) when event listener is triggered
                        cmd = vdom_cmd_rx.receive() => { // 2) when global state is changed or injected window.document event is emitted
                            if let Some(cmd) = cmd {
                                match cmd {
                                    VDomCommand::UpdateDom => {}
                                    VDomCommand::GlobalState(state) => {
                                        state.handler(root.clone())
                                    }
                                }
                            }
                        }
                    }

                    let muts = vdom.work_with_deadline(|| false);
                    for edit in muts {
                        edit_queue
                            .lock()
                            .unwrap()
                            .push(serde_json::to_string(&edit.edits).unwrap());
                    }

                    proxy
                        .send_event(UiEvent::WindowEvent(WindowEvent::Update))
                        .unwrap();
                }
            });
        });
    }

    fn handle_initial_window_events(world: &mut World)
    where
        CoreCommand: 'static + Send + Sync + Clone + Debug,
        Props: 'static + Send + Sync + Clone,
    {
        let world = world.cell();
        let mut dioxus_windows = world.get_non_send_resource_mut::<DioxusWindows>().unwrap();
        let mut bevy_windows = world.get_resource_mut::<Windows>().unwrap();
        let mut create_window_events = world.get_resource_mut::<Events<CreateWindow>>().unwrap();
        let mut window_created_events = world.get_resource_mut::<Events<WindowCreated>>().unwrap();

        for create_window_event in create_window_events.drain() {
            let window = dioxus_windows.create::<CoreCommand, Props>(
                &world,
                create_window_event.id,
                &create_window_event.descriptor,
            );
            bevy_windows.add(window);
            window_created_events.send(WindowCreated {
                id: create_window_event.id,
            });
        }
    }
}

fn send_ui_commands<UiCommand>(mut events: EventReader<UiCommand>, tx: Res<Sender<UiCommand>>)
where
    UiCommand: 'static + Send + Sync + Clone + Debug,
{
    for cmd in events.iter() {
        match tx.try_send(cmd.clone()) {
            Ok(()) => {}
            Err(e) => match e {
                TrySendError::Full(e) => {
                    error!("Failed to send UiCommand: channel is full: event: {:?}", e);
                }
                TrySendError::Closed(e) => {
                    error!(
                        "Failed to send UiCommand: channel is closed: event: {:?}",
                        e
                    );
                }
            },
        }
    }
}

fn change_window(
    mut dioxus_windows: NonSendMut<DioxusWindows>,
    mut windows: ResMut<Windows>,
    mut window_dpi_changed_events: EventWriter<WindowScaleFactorChanged>,
    mut window_close_events: EventWriter<WindowClosed>,
) {
    let mut removed_windows = vec![];
    for bevy_window in windows.iter_mut() {
        let id = bevy_window.id();
        let window = dioxus_windows.get_tao_window(id).unwrap();

        for command in bevy_window.drain_commands() {
            match command {
                WindowCommand::SetWindowMode {
                    mode,
                    resolution: UVec2 { x, y },
                } => match mode {
                    WindowMode::BorderlessFullscreen => {
                        window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                    }
                    WindowMode::Fullscreen => {
                        window.set_fullscreen(Some(Fullscreen::Exclusive(
                            DioxusWindows::get_best_videomode(&window.current_monitor().unwrap()),
                        )));
                    }
                    WindowMode::SizedFullscreen => window.set_fullscreen(Some(
                        Fullscreen::Exclusive(DioxusWindows::get_fitting_videomode(
                            &window.current_monitor().unwrap(),
                            x,
                            y,
                        )),
                    )),
                    WindowMode::Windowed => window.set_fullscreen(None),
                },
                WindowCommand::SetTitle { title } => {
                    window.set_title(&title);
                }
                WindowCommand::SetScaleFactor { scale_factor } => {
                    window_dpi_changed_events.send(WindowScaleFactorChanged { id, scale_factor });
                }
                WindowCommand::SetResolution {
                    logical_resolution:
                        Vec2 {
                            x: width,
                            y: height,
                        },
                    scale_factor,
                } => {
                    window.set_inner_size(
                        LogicalSize::new(width, height).to_physical::<f64>(scale_factor),
                    );
                }
                WindowCommand::SetPresentMode { .. } => (),
                WindowCommand::SetResizable { resizable } => {
                    window.set_resizable(resizable);
                }
                WindowCommand::SetDecorations { decorations } => {
                    window.set_decorations(decorations);
                }
                WindowCommand::SetCursorIcon { icon } => {
                    window.set_cursor_icon(converter::convert_cursor_icon(icon));
                }
                WindowCommand::SetCursorLockMode { locked } => {
                    window
                        .set_cursor_grab(locked)
                        .unwrap_or_else(|e| error!("Unable to un/grab cursor: {}", e));
                }
                WindowCommand::SetCursorVisibility { visible } => {
                    window.set_cursor_visible(visible);
                }
                WindowCommand::SetCursorPosition { position } => {
                    let inner_size = window.inner_size().to_logical::<f32>(window.scale_factor());
                    window
                        .set_cursor_position(LogicalPosition::new(
                            position.x,
                            inner_size.height - position.y,
                        ))
                        .unwrap_or_else(|e| error!("Unable to set cursor position: {}", e));
                }
                WindowCommand::SetMaximized { maximized } => {
                    window.set_maximized(maximized);
                }
                WindowCommand::SetMinimized { minimized } => {
                    window.set_minimized(minimized);
                }
                WindowCommand::SetPosition { position } => {
                    window.set_outer_position(PhysicalPosition {
                        x: position[0],
                        y: position[1],
                    });
                }
                WindowCommand::Center(monitor_selection) => {
                    use bevy::window::MonitorSelection::*;
                    let maybe_monitor = match monitor_selection {
                        Current => window.current_monitor(),
                        Primary => window.primary_monitor(),
                        Number(n) => window.available_monitors().nth(n),
                    };

                    if let Some(monitor) = maybe_monitor {
                        let screen_size = monitor.size();

                        let window_size = window.outer_size();

                        window.set_outer_position(PhysicalPosition {
                            x: screen_size.width.saturating_sub(window_size.width) as f64 / 2.
                                + monitor.position().x as f64,
                            y: screen_size.height.saturating_sub(window_size.height) as f64 / 2.
                                + monitor.position().y as f64,
                        });
                    } else {
                        warn!("Couldn't get monitor selected with: {monitor_selection:?}");
                    }
                }
                WindowCommand::SetResizeConstraints { resize_constraints } => {
                    let constraints = resize_constraints.check_constraints();
                    let min_inner_size = LogicalSize {
                        width: constraints.min_width,
                        height: constraints.min_height,
                    };
                    let max_inner_size = LogicalSize {
                        width: constraints.max_width,
                        height: constraints.max_height,
                    };

                    window.set_min_inner_size(Some(min_inner_size));
                    if constraints.max_width.is_finite() && constraints.max_height.is_finite() {
                        window.set_max_inner_size(Some(max_inner_size));
                    }
                }
                WindowCommand::Close => {
                    removed_windows.push(id);
                    break;
                }
            }
        }
    }

    if !removed_windows.is_empty() {
        for id in removed_windows {
            let _ = dioxus_windows.remove(id);
            windows.remove(id);
            window_close_events.send(WindowClosed { id });
        }
    }
}
struct UiStagePlugin;

impl Plugin for UiStagePlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::PostUpdate,
            UiStage::First,
            SystemStage::parallel(),
        )
        .add_stage_after(UiStage::First, UiStage::PreUpdate, SystemStage::parallel())
        .add_stage_after(UiStage::PreUpdate, UiStage::Update, SystemStage::parallel())
        .add_stage_after(
            UiStage::Update,
            UiStage::PostUpdate,
            SystemStage::parallel(),
        )
        .add_stage_after(UiStage::PostUpdate, UiStage::Last, SystemStage::parallel());
    }
}
