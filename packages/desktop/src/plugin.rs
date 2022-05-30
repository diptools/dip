use crate::{
    converter,
    event::{
        DomUpdated, KeyboardEvent, MaximizeToggled, UiEvent, UpdateDom, WindowDragged,
        WindowMaximized, WindowMinimized,
    },
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
use dioxus_core::Component as DioxusComponent;
use futures_intrusive::channel::{
    shared::{channel, Sender},
    TrySendError,
};
use std::{fmt::Debug, marker::PhantomData};
use tokio::runtime::Runtime;
use wry::application::{
    dpi::{LogicalPosition, LogicalSize, PhysicalPosition},
    event_loop::EventLoop,
    window::Fullscreen,
};

pub struct DioxusPlugin<CoreCommand, UiCommand, Props = ()> {
    pub root: DioxusComponent<Props>,
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
        let runtime = Runtime::new().unwrap();

        let (core_tx, core_rx) = channel::<CoreCommand>(8);
        let (ui_tx, ui_rx) = channel::<UiCommand>(8);
        let (dom_update_tx, dom_update_rx) = channel::<UpdateDom>(8);
        let settings = app
            .world
            .remove_non_send_resource::<DioxusSettings<Props>>()
            .unwrap_or_default();

        let event_loop = EventLoop::<UiEvent<CoreCommand>>::with_user_event();
        app.add_plugin(WindowPlugin::default())
            .add_plugin(InputPlugin)
            .add_event::<KeyboardEvent>()
            .add_event::<CoreCommand>()
            .add_event::<UiCommand>()
            .add_event::<DomUpdated>()
            .add_event::<WindowDragged>()
            .add_event::<WindowMinimized>()
            .add_event::<WindowMaximized>()
            .add_event::<MaximizeToggled>()
            .insert_resource(core_tx)
            .insert_resource(core_rx)
            .insert_resource(ui_tx)
            .insert_resource(ui_rx)
            .insert_resource(dom_update_tx)
            .insert_resource(dom_update_rx)
            .insert_resource(runtime)
            .insert_resource(self.root)
            .insert_non_send_resource(settings)
            .init_non_send_resource::<DioxusWindows>()
            .set_runner(|app| runner::<CoreCommand, UiCommand, Props>(app))
            .insert_non_send_resource(event_loop)
            .add_system_to_stage(CoreStage::Last, send_ui_commands::<UiCommand>)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                change_window, /* TODO.label(ModifiesWindows) // is recentry introduced ( > 0.7 ) */
            );

        Self::handle_initial_window_events(&mut app.world);
    }
}

impl<CoreCommand, UiCommand, Props> DioxusPlugin<CoreCommand, UiCommand, Props> {
    pub fn new(root: DioxusComponent<Props>) -> Self {
        Self {
            root,
            core_cmd_type: PhantomData,
            ui_cmd_type: PhantomData,
        }
    }

    fn handle_initial_window_events(world: &mut World)
    where
        CoreCommand: 'static + Send + Sync + Clone + Debug,
        UiCommand: 'static + Send + Sync + Clone + Debug,
        Props: 'static + Send + Sync + Clone,
    {
        let world = world.cell();
        let mut dioxus_windows = world.get_non_send_mut::<DioxusWindows>().unwrap();
        let mut bevy_windows = world.get_resource_mut::<Windows>().unwrap();
        let mut create_window_events = world.get_resource_mut::<Events<CreateWindow>>().unwrap();
        let mut window_created_events = world.get_resource_mut::<Events<WindowCreated>>().unwrap();

        for create_window_event in create_window_events.drain() {
            let window = dioxus_windows.create::<CoreCommand, UiCommand, Props>(
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
    for e in events.iter() {
        match tx.try_send(e.clone()) {
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
