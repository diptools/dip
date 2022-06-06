//! Dioxus Plugin for Bevy
#![allow(non_snake_case)]

use crate::{
    context::UiContext,
    converter,
    event::{KeyboardEvent, UiEvent, VDomCommand, WindowEvent},
    runner::runner,
    setting::DioxusSettings,
    window::DioxusWindows,
};
use bevy::{
    app::prelude::*,
    ecs::{event::Events, prelude::*},
    input::InputPlugin,
    log::error,
    window::{
        CreateWindow, WindowCommand, WindowCreated, WindowMode, WindowPlugin,
        WindowScaleFactorChanged, Windows,
    },
};
use dioxus_core::{Component as DioxusComponent, SchedulerMsg, VirtualDom};
use fermi::{AtomId, AtomRoot};
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
pub struct DioxusPlugin<CoreCommand, UiCommand, Props = ()> {
    /// Root component
    pub Root: DioxusComponent<Props>,
    core_cmd_type: PhantomData<CoreCommand>,
    ui_cmd_type: PhantomData<UiCommand>,
}

impl<CoreCommand, UiCommand, Props> Plugin for DioxusPlugin<CoreCommand, UiCommand, Props>
where
    CoreCommand: 'static + Send + Sync + Clone + Debug,
    UiCommand: 'static + Send + Sync + Clone + Debug,
    Props: 'static + Send + Sync + Clone + Default,
{
    fn build(&self, app: &mut App) {
        let (core_tx, core_rx) = channel::<CoreCommand>(8);
        let (ui_tx, ui_rx) = channel::<UiCommand>(8);
        let (vdom_cmd_tx, vdom_cmd_rx) = channel::<VDomCommand>(8);

        let settings = app
            .world
            .remove_non_send_resource::<DioxusSettings<Props>>()
            .unwrap_or_default();

        app.add_plugin(WindowPlugin::default())
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
            .add_system_to_stage(CoreStage::PostUpdate, send_ui_commands::<UiCommand>)
            .add_system_to_stage(CoreStage::PostUpdate, rerender_dom)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                change_window, /* TODO.label(ModifiesWindows) // is recentry introduced ( > 0.7 ) */
            );

        self.spawn_virtual_dom(&mut app.world, (core_tx, ui_rx), vdom_cmd_rx);
        Self::handle_initial_window_events(&mut app.world);
    }
}

impl<CoreCommand, UiCommand, Props> DioxusPlugin<CoreCommand, UiCommand, Props>
where
    CoreCommand: Clone + Debug + Send + Sync,
    UiCommand: Clone + Debug + Send + Sync,
    Props: Send + Sync + Clone + 'static,
{
    /// Initialize DioxusPlugin with root component and channel types
    /// ```
    /// use bevy_dioxus::desktop::prelude::*;
    /// use dioxus::prelude::*;
    ///
    /// // DioxusPlugin accepts any types as command. Pass empty tuple if channel is not necessary.
    /// type CoreCommand = ();
    /// type UiCommand = ();
    ///
    /// fn main() {
    ///    App::new()
    ///         .add_plugin(DioxusPlugin::<CoreCommand, UiCommand>::new(Root))
    ///         .run();
    /// }
    ///
    /// fn Root(cx: Scope) -> Element {
    ///    cx.render(rsx! {
    ///    h1 { "<Root /> Component" }
    ///        })
    /// }
    /// ```
    pub fn new(Root: DioxusComponent<Props>) -> Self {
        Self {
            Root,
            core_cmd_type: PhantomData,
            ui_cmd_type: PhantomData,
        }
    }

    fn spawn_virtual_dom(
        &self,
        world: &mut World,
        (core_tx, ui_rx): (Sender<CoreCommand>, Receiver<UiCommand>),
        vdom_cmd_rx: Receiver<VDomCommand>,
    ) {
        println!("spawn_virtual_dom");
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

            Runtime::new().unwrap().block_on(async move {
                loop {
                    select! {
                        () = vdom.wait_for_work() => {
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
                        cmd = vdom_cmd_rx.receive() => {
                            if let Some(cmd) = cmd {
                                match cmd {
                                    VDomCommand::UpdateDom => {
                                    }
                                    VDomCommand::GlobalState(state) => {
                                        let cx = vdom.base_scope();
                                        let _root = match cx.consume_context::<Rc<AtomRoot>>() {
                                            Some(root) => root,
                                            None => cx.provide_root_context(Rc::new(AtomRoot::new(
                                                cx.schedule_update_any(),
                                            ))),
                                        };
                                        println!("set atom id: {:?}, value: {:?}",state.id as AtomId, state.value);
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
                        }
                    }
                }
            });
        });
    }

    fn handle_initial_window_events(world: &mut World)
    where
        CoreCommand: 'static + Send + Sync + Clone + Debug,
        Props: 'static + Send + Sync + Clone,
    {
        println!("handle_initial_window_events");
        let world = world.cell();
        let mut dioxus_windows = world.get_non_send_mut::<DioxusWindows>().unwrap();
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

fn rerender_dom(tx: Res<Sender<VDomCommand>>) {
    match tx.try_send(VDomCommand::UpdateDom) {
        Ok(()) => {}
        Err(e) => match e {
            TrySendError::Full(e) => {
                error!(
                    "Failed to send VDomCommand: channel is full: event: {:?}",
                    e
                );
            }
            TrySendError::Closed(e) => {
                error!(
                    "Failed to send VDomCommand: channel is closed: event: {:?}",
                    e
                );
            }
        },
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
    dioxus_windows: NonSend<DioxusWindows>,
    mut windows: ResMut<Windows>,
    mut window_dpi_changed_events: EventWriter<WindowScaleFactorChanged>,
    // mut window_close_events: EventWriter<WindowClosed>, // bevy > 0.7
) {
    // let mut removed_windows = vec![];

    for bevy_window in windows.iter_mut() {
        let id = bevy_window.id();
        let window = dioxus_windows.get_tao_window(id).unwrap();

        for command in bevy_window.drain_commands() {
            match command {
                WindowCommand::SetWindowMode {
                    mode,
                    resolution: (width, height),
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
                            width,
                            height,
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
                    logical_resolution: (width, height),
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
                } // WindowCommand::Close => {
                  //     removed_windows.push(id);
                  //     break;
                  // }
            }
        }
    }

    // if !removed_windows.is_empty() {
    //     for id in removed_windows {
    //         let _ = dioxus_windows.remove_window(id);
    //         windows.remove(id);
    //         window_close_events.send(WindowClosed { id });
    //     }
    // }
}
