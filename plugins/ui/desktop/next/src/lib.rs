mod converters;
mod system;
mod tao_config;
mod tao_windows;

pub mod prelude {
    pub use crate::DesktopPlugin;
}

use bevy::{
    // a11y::{AccessibilityPlugin, AccessibilityRequested},
    app::{App, AppExit, CoreSet, Plugin},
    ecs::{
        change_detection::DetectChanges,
        entity::Entity,
        event::{EventWriter, Events, ManualEventReader},
        query::Added,
        schedule::{IntoSystemConfig, IntoSystemConfigs},
        system::{Commands, NonSend, NonSendMut, Query, Res, SystemParam, SystemState},
        world::FromWorld,
    },
    input::{
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseMotion, MouseScrollUnit, MouseWheel},
        touch::TouchInput,
    },
    math::{ivec2, DVec2, Vec2},
    utils::{
        tracing::{trace, warn},
        Instant,
    },
    window::{
        exit_on_all_closed, CursorEntered, CursorLeft, CursorMoved, FileDragAndDrop, RequestRedraw,
        Window, WindowBackendScaleFactorChanged, WindowCloseRequested, WindowCreated,
        WindowFocused, WindowMoved, WindowResized, WindowScaleFactorChanged,
    },
};
use dioxus::core::Component as DioxusComponent;
use system::{changed_window, create_window, despawn_window, CachedWindow};
use tao_config::*;
use tao_windows::*;
use wry::application::{
    self as tao,
    event::{self, DeviceEvent, Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
};

/// Dioxus Plugin for Bevy.
/// Recemble to [bevy_winit](https://github.com/bevyengine/bevy/blob/v0.10.1/crates/bevy_winit/src/lib.rs). It uses `wry` instead of `winit` as a window manager.
pub struct DesktopPlugin<
    // UiState,
    // UiAction,
    // AsyncAction,
    RootProps = (),
> {
    /// Root Dioxus component
    pub root_component: DioxusComponent<RootProps>,
    // ui_state_type: PhantomData<UiState>,
    // ui_action_type: PhantomData<UiAction>,
    // async_action_type: PhantomData<AsyncAction>,
}

impl<
        // UiState,
        // UiAction,
        // AsyncAction,
        RootProps,
    > Plugin
    for DesktopPlugin<
        // UiState,
        // UiAction,
        // AsyncAction,
        RootProps,
    >
where
    // UiState: 'static + Send + Sync + UiStateHandler,
    // UiAction: 'static + Send + Sync + Clone + Debug,
    // AsyncAction: 'static + Send + Sync + Clone + Debug,
    RootProps: 'static + Send + Sync + Clone + Default,
{
    fn build(&self, app: &mut App) {
        // Create `tao` based window event loop
        let event_loop = EventLoop::<
            UiEvent, // <UiAction, AsyncAction>
        >::with_user_event();

        // TODO: https://github.com/bevyengine/bevy/blob/v0.10.1/crates/bevy_winit/src/lib.rs#L59..L68
        // #[cfg(target_os = "android")]
        // {
        //     use winit::platform::android::EventLoopBuilderExtAndroid;
        //     event_loop_builder.with_android_app(
        //         ANDROID_APP
        //             .get()
        //             .expect("Bevy must be setup with the #[bevy_main] macro on Android")
        //             .clone(),
        //     );
        // }
        //

        app.insert_non_send_resource(event_loop);

        app.init_non_send_resource::<TaoWindows>()
            .init_resource::<TaoSettings>()
            .set_runner(tao_runner)
            // exit_on_all_closed only uses the query to determine if the query is empty,
            // and so doesn't care about ordering relative to changed_window
            .add_systems(
                (
                    changed_window.ambiguous_with(exit_on_all_closed),
                    // Update the state of the window before attempting to despawn to ensure consistent event ordering
                    despawn_window.after(changed_window),
                )
                    .in_base_set(CoreSet::Last),
            );

        // TODO
        // app.add_plugin(AccessibilityPlugin);

        // TODO: https://github.com/bevyengine/bevy/blob/v0.10.1/crates/bevy_winit/src/lib.rs#L89..L90
        // #[cfg(target_arch = "wasm32")]
        // app.add_plugin(CanvasParentResizePlugin);

        #[cfg(not(target_arch = "wasm32"))]
        let mut create_window_system_state: SystemState<(
            Commands,
            NonSendMut<EventLoop<()>>,
            Query<(Entity, &mut Window)>,
            EventWriter<WindowCreated>,
            NonSendMut<TaoWindows>,
            // NonSendMut<AccessKitAdapters>,
            // ResMut<TaoActionHandlers>,
            // ResMut<AccessibilityRequested>,
        )> = SystemState::from_world(&mut app.world);

        // TODO: https://github.com/bevyengine/bevy/blob/v0.10.1/crates/bevy_winit/src/lib.rs#L104..L115
        // #[cfg(target_arch = "wasm32")]
        // let mut create_window_system_state: SystemState<(
        //     Commands,
        //     NonSendMut<EventLoop<()>>,
        //     Query<(Entity, &mut Window)>,
        //     EventWriter<WindowCreated>,
        //     NonSendMut<TaoWindows>,
        //     NonSendMut<AccessKitAdapters>,
        //     ResMut<TaoActionHandlers>,
        //     ResMut<AccessibilityRequested>,
        //     ResMut<CanvasParentResizeEventChannel>,
        // )> = SystemState::from_world(&mut app.world);

        // TODO: https://github.com/bevyengine/bevy/blob/v0.10.1/crates/bevy_winit/src/lib.rs#L117..L161
        // // And for ios and macos, we should not create window early, all ui related code should be executed inside
        // // UIApplicationMain/NSApplicationMain.
        // #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "macos")))]
        // {
        //     #[cfg(not(target_arch = "wasm32"))]
        //     let (
        //         commands,
        //         event_loop,
        //         mut new_windows,
        //         event_writer,
        //         tao_windows,
        //         adapters,
        //         handlers,
        //         accessibility_requested,
        //     ) = create_window_system_state.get_mut(&mut app.world);

        //     #[cfg(target_arch = "wasm32")]
        //     let (
        //         commands,
        //         event_loop,
        //         mut new_windows,
        //         event_writer,
        //         tao_windows,
        //         adapters,
        //         handlers,
        //         accessibility_requested,
        //         event_channel,
        //     ) = create_window_system_state.get_mut(&mut app.world);

        //     // Here we need to create a tao-window and give it a WindowHandle which the renderer can use.
        //     // It needs to be spawned before the start of the startup schedule, so we cannot use a regular system.
        //     // Instead we need to create the window and spawn it using direct world access
        //     create_window(
        //         commands,
        //         &event_loop,
        //         new_windows.iter_mut(),
        //         event_writer,
        //         tao,
        //         adapters,
        //         handlers,
        //         accessibility_requested,
        //         #[cfg(target_arch = "wasm32")]
        //         event_channel,
        //     );
        // }

        create_window_system_state.apply(&mut app.world);
    }
}

fn run<F>(event_loop: EventLoop<()>, event_handler: F) -> !
where
    F: 'static + FnMut(Event<'_, ()>, &EventLoopWindowTarget<()>, &mut ControlFlow),
{
    event_loop.run(event_handler)
}

// TODO: It may be worth moving this cfg into a procedural macro so that it can be referenced by
// a single name instead of being copied around.
// https://gist.github.com/jakerr/231dee4a138f7a5f25148ea8f39b382e seems to work.
#[cfg(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
fn run_return<F>(event_loop: &mut EventLoop<()>, event_handler: F)
where
    F: FnMut(Event<'_, ()>, &EventLoopWindowTarget<()>, &mut ControlFlow),
{
    use tao::platform::run_return::EventLoopExtRunReturn;
    event_loop.run_return(event_handler);
}

// TODO: https://github.com/bevyengine/bevy/blob/v0.10.1/crates/bevy_winit/src/lib.rs#L194..L208
// #[cfg(not(any(
//     target_os = "windows",
//     target_os = "macos",
//     target_os = "linux",
//     target_os = "dragonfly",
//     target_os = "freebsd",
//     target_os = "netbsd",
//     target_os = "openbsd"
// )))]
// fn run_return<F>(_event_loop: &mut EventLoop<()>, _event_handler: F)
// where
//     F: FnMut(Event<'_, ()>, &EventLoopWindowTarget<()>, &mut ControlFlow),
// {
//     panic!("Run return is not supported on this platform!")
// }

#[derive(SystemParam)]
struct WindowEvents<'w> {
    window_resized: EventWriter<'w, WindowResized>,
    window_close_requested: EventWriter<'w, WindowCloseRequested>,
    window_scale_factor_changed: EventWriter<'w, WindowScaleFactorChanged>,
    window_backend_scale_factor_changed: EventWriter<'w, WindowBackendScaleFactorChanged>,
    window_focused: EventWriter<'w, WindowFocused>,
    window_moved: EventWriter<'w, WindowMoved>,
}

#[derive(SystemParam)]
struct InputEvents<'w> {
    keyboard_input: EventWriter<'w, KeyboardInput>,
    // character_input: EventWriter<'w, ReceivedCharacter>,
    mouse_button_input: EventWriter<'w, MouseButtonInput>,
    mouse_wheel_input: EventWriter<'w, MouseWheel>,
    touch_input: EventWriter<'w, TouchInput>,
    // ime_input: EventWriter<'w, Ime>,
}

#[derive(SystemParam)]
struct CursorEvents<'w> {
    cursor_moved: EventWriter<'w, CursorMoved>,
    cursor_entered: EventWriter<'w, CursorEntered>,
    cursor_left: EventWriter<'w, CursorLeft>,
}

// https://github.com/bevyengine/bevy/blob/v0.10.1/crates/bevy_winit/src/lib.rs#L237..L246
// #[cfg(any(
//     target_os = "linux",
//     target_os = "dragonfly",
//     target_os = "freebsd",
//     target_os = "netbsd",
//     target_os = "openbsd"
// ))]
// pub fn tao_runner_any_thread(app: App) {
//     tao_runner_with(app, EventLoop::new_any_thread());
// }

/// Stores state that must persist between frames.
struct TaoPersistentState {
    /// Tracks whether or not the application is active or suspended.
    active: bool,
    /// Tracks whether or not an event has occurred this frame that would trigger an update in low
    /// power mode. Should be reset at the end of every frame.
    low_power_event: bool,
    /// Tracks whether the event loop was started this frame because of a redraw request.
    redraw_request_sent: bool,
    /// Tracks if the event loop was started this frame because of a `WaitUntil` timeout.
    timeout_reached: bool,
    last_update: Instant,
}
impl Default for TaoPersistentState {
    fn default() -> Self {
        Self {
            active: false,
            low_power_event: false,
            redraw_request_sent: false,
            timeout_reached: false,
            last_update: Instant::now(),
        }
    }
}

pub fn tao_runner(mut app: App) {
    // We remove this so that we have ownership over it.
    let mut event_loop = app
        .world
        .remove_non_send_resource::<EventLoop<()>>()
        .unwrap();

    let mut app_exit_event_reader = ManualEventReader::<AppExit>::default();
    let mut redraw_event_reader = ManualEventReader::<RequestRedraw>::default();
    let mut tao_state = TaoPersistentState::default();
    app.world
        .insert_non_send_resource(event_loop.create_proxy());

    let return_from_run = app.world.resource::<TaoSettings>().return_from_run;

    trace!("Entering tao event loop");

    let mut focused_window_state: SystemState<(Res<TaoSettings>, Query<&Window>)> =
        SystemState::from_world(&mut app.world);

    #[cfg(not(target_arch = "wasm32"))]
    let mut create_window_system_state: SystemState<(
        Commands,
        Query<(Entity, &mut Window), Added<Window>>,
        EventWriter<WindowCreated>,
        NonSendMut<TaoWindows>,
        // NonSendMut<AccessKitAdapters>,
        // ResMut<TaoActionHandlers>,
        // ResMut<AccessibilityRequested>,
    )> = SystemState::from_world(&mut app.world);

    // TODO
    // #[cfg(target_arch = "wasm32")]
    // let mut create_window_system_state: SystemState<(
    //     Commands,
    //     Query<(Entity, &mut Window), Added<Window>>,
    //     EventWriter<WindowCreated>,
    //     NonSendMut<TaoWindows>,
    //     NonSendMut<AccessKitAdapters>,
    //     ResMut<TaoActionHandlers>,
    //     ResMut<AccessibilityRequested>,
    //     ResMut<CanvasParentResizeEventChannel>,
    // )> = SystemState::from_world(&mut app.world);

    let event_handler = move |event: Event<()>,
                              event_loop: &EventLoopWindowTarget<()>,
                              control_flow: &mut ControlFlow| {
        #[cfg(feature = "trace")]
        let _span = bevy::utils::tracing::info_span!("tao event_handler").entered();

        if let Some(app_exit_events) = app.world.get_resource::<Events<AppExit>>() {
            if app_exit_event_reader.iter(app_exit_events).last().is_some() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        match event {
            event::Event::NewEvents(start) => {
                let (tao_config, window_focused_query) = focused_window_state.get(&app.world);

                let app_focused = window_focused_query.iter().any(|window| window.focused);

                // Check if either the `WaitUntil` timeout was triggered by tao, or that same
                // amount of time has elapsed since the last app update. This manual check is needed
                // because we don't know if the criteria for an app update were met until the end of
                // the frame.
                let auto_timeout_reached = matches!(start, StartCause::ResumeTimeReached { .. });
                let now = Instant::now();
                let manual_timeout_reached = match tao_config.update_mode(app_focused) {
                    UpdateMode::Continuous => false,
                    UpdateMode::Reactive { max_wait }
                    | UpdateMode::ReactiveLowPower { max_wait } => {
                        now.duration_since(tao_state.last_update) >= *max_wait
                    }
                };
                // The low_power_event state and timeout must be reset at the start of every frame.
                tao_state.low_power_event = false;
                tao_state.timeout_reached = auto_timeout_reached || manual_timeout_reached;
            }
            event::Event::WindowEvent {
                event,
                window_id: tao_window_id,
                ..
            } => {
                // Fetch and prepare details from the world
                let mut system_state: SystemState<(
                    NonSend<TaoWindows>,
                    Query<(&mut Window, &mut CachedWindow)>,
                    WindowEvents,
                    InputEvents,
                    CursorEvents,
                    EventWriter<FileDragAndDrop>,
                )> = SystemState::new(&mut app.world);
                let (
                    tao_windows,
                    mut window_query,
                    mut window_events,
                    mut input_events,
                    mut cursor_events,
                    mut file_drag_and_drop_events,
                ) = system_state.get_mut(&mut app.world);

                // Entity of this window
                let window_entity =
                    if let Some(entity) = tao_windows.get_window_entity(tao_window_id) {
                        entity
                    } else {
                        warn!(
                            "Skipped event {:?} for unknown tao Window Id {:?}",
                            event, tao_window_id
                        );
                        return;
                    };

                let (mut window, mut cache) =
                    if let Ok((window, info)) = window_query.get_mut(window_entity) {
                        (window, info)
                    } else {
                        warn!(
                            "Window {:?} is missing `Window` component, skipping event {:?}",
                            window_entity, event
                        );
                        return;
                    };

                tao_state.low_power_event = true;

                match event {
                    WindowEvent::Resized(size) => {
                        window
                            .resolution
                            .set_physical_resolution(size.width, size.height);

                        window_events.window_resized.send(WindowResized {
                            window: window_entity,
                            width: window.width(),
                            height: window.height(),
                        });
                    }
                    WindowEvent::CloseRequested => {
                        window_events
                            .window_close_requested
                            .send(WindowCloseRequested {
                                window: window_entity,
                            });
                    }
                    WindowEvent::KeyboardInput { ref event, .. } => {
                        input_events
                            .keyboard_input
                            .send(converters::convert_keyboard_input(event));
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let physical_position = DVec2::new(
                            position.x,
                            // Flip the coordinate space from tao's context to our context.
                            window.resolution.physical_height() as f64 - position.y,
                        );

                        window.set_physical_cursor_position(Some(physical_position));

                        cursor_events.cursor_moved.send(CursorMoved {
                            window: window_entity,
                            position: (physical_position / window.resolution.scale_factor())
                                .as_vec2(),
                        });
                    }
                    WindowEvent::CursorEntered { .. } => {
                        cursor_events.cursor_entered.send(CursorEntered {
                            window: window_entity,
                        });
                    }
                    WindowEvent::CursorLeft { .. } => {
                        window.set_physical_cursor_position(None);

                        cursor_events.cursor_left.send(CursorLeft {
                            window: window_entity,
                        });
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        input_events.mouse_button_input.send(MouseButtonInput {
                            button: converters::convert_mouse_button(button),
                            state: converters::convert_element_state(state),
                        });
                    }
                    WindowEvent::MouseWheel { delta, .. } => match delta {
                        event::MouseScrollDelta::LineDelta(x, y) => {
                            input_events.mouse_wheel_input.send(MouseWheel {
                                unit: MouseScrollUnit::Line,
                                x,
                                y,
                            });
                        }
                        event::MouseScrollDelta::PixelDelta(p) => {
                            input_events.mouse_wheel_input.send(MouseWheel {
                                unit: MouseScrollUnit::Pixel,
                                x: p.x as f32,
                                y: p.y as f32,
                            });
                        }
                        _ => unreachable!(),
                    },
                    WindowEvent::Touch(touch) => {
                        let location = touch.location.to_logical(window.resolution.scale_factor());

                        // Event
                        input_events
                            .touch_input
                            .send(converters::convert_touch_input(touch, location));
                    }
                    // WindowEvent::ReceivedCharacter(c) => {
                    //     input_events.character_input.send(ReceivedCharacter {
                    //         window: window_entity,
                    //         char: c,
                    //     });
                    // }
                    WindowEvent::ScaleFactorChanged {
                        scale_factor,
                        new_inner_size,
                    } => {
                        window_events.window_backend_scale_factor_changed.send(
                            WindowBackendScaleFactorChanged {
                                window: window_entity,
                                scale_factor,
                            },
                        );

                        let prior_factor = window.resolution.scale_factor();
                        window.resolution.set_scale_factor(scale_factor);
                        let new_factor = window.resolution.scale_factor();

                        if let Some(forced_factor) = window.resolution.scale_factor_override() {
                            // If there is a scale factor override, then force that to be used
                            // Otherwise, use the OS suggested size
                            // We have already told the OS about our resize constraints, so
                            // the new_inner_size should take those into account
                            *new_inner_size =
                                tao::dpi::LogicalSize::new(window.width(), window.height())
                                    .to_physical::<u32>(forced_factor);
                            // TODO: Should this not trigger a WindowsScaleFactorChanged?
                        } else if approx::relative_ne!(new_factor, prior_factor) {
                            // Trigger a change event if they are approximately different
                            window_events.window_scale_factor_changed.send(
                                WindowScaleFactorChanged {
                                    window: window_entity,
                                    scale_factor,
                                },
                            );
                        }

                        let new_logical_width = (new_inner_size.width as f64 / new_factor) as f32;
                        let new_logical_height = (new_inner_size.height as f64 / new_factor) as f32;
                        if approx::relative_ne!(window.width(), new_logical_width)
                            || approx::relative_ne!(window.height(), new_logical_height)
                        {
                            window_events.window_resized.send(WindowResized {
                                window: window_entity,
                                width: new_logical_width,
                                height: new_logical_height,
                            });
                        }
                        window
                            .resolution
                            .set_physical_resolution(new_inner_size.width, new_inner_size.height);
                    }
                    WindowEvent::Focused(focused) => {
                        // Component
                        window.focused = focused;

                        window_events.window_focused.send(WindowFocused {
                            window: window_entity,
                            focused,
                        });
                    }
                    WindowEvent::DroppedFile(path_buf) => {
                        file_drag_and_drop_events.send(FileDragAndDrop::DroppedFile {
                            window: window_entity,
                            path_buf,
                        });
                    }
                    WindowEvent::HoveredFile(path_buf) => {
                        file_drag_and_drop_events.send(FileDragAndDrop::HoveredFile {
                            window: window_entity,
                            path_buf,
                        });
                    }
                    WindowEvent::HoveredFileCancelled => {
                        file_drag_and_drop_events.send(FileDragAndDrop::HoveredFileCancelled {
                            window: window_entity,
                        });
                    }
                    WindowEvent::Moved(position) => {
                        let position = ivec2(position.x, position.y);

                        window.position.set(position);

                        window_events.window_moved.send(WindowMoved {
                            entity: window_entity,
                            position,
                        });
                    }
                    // WindowEvent::Ime(event) => match event {
                    //     event::Ime::Preedit(value, cursor) => {
                    //         input_events.ime_input.send(Ime::Preedit {
                    //             window: window_entity,
                    //             value,
                    //             cursor,
                    //         });
                    //     }
                    //     event::Ime::Commit(value) => input_events.ime_input.send(Ime::Commit {
                    //         window: window_entity,
                    //         value,
                    //     }),
                    //     event::Ime::Enabled => input_events.ime_input.send(Ime::Enabled {
                    //         window: window_entity,
                    //     }),
                    //     event::Ime::Disabled => input_events.ime_input.send(Ime::Disabled {
                    //         window: window_entity,
                    //     }),
                    // },
                    _ => {}
                }

                if window.is_changed() {
                    cache.window = window.clone();
                }
            }
            event::Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta: (x, y), .. },
                ..
            } => {
                let mut system_state: SystemState<EventWriter<MouseMotion>> =
                    SystemState::new(&mut app.world);
                let mut mouse_motion = system_state.get_mut(&mut app.world);

                mouse_motion.send(MouseMotion {
                    delta: Vec2::new(x as f32, y as f32),
                });
            }
            event::Event::Suspended => {
                tao_state.active = false;
                #[cfg(target_os = "android")]
                {
                    // Bevy doesn't support suspend/resume so we just exit
                    // and Android will restart the application on resume
                    // TODO: Save save some state and load on resume
                    *control_flow = ControlFlow::Exit;
                }
            }
            event::Event::Resumed => {
                tao_state.active = true;
            }
            event::Event::MainEventsCleared => {
                let (tao_config, window_focused_query) = focused_window_state.get(&app.world);

                let update = if tao_state.active {
                    // True if _any_ windows are currently being focused
                    let app_focused = window_focused_query.iter().any(|window| window.focused);
                    match tao_config.update_mode(app_focused) {
                        UpdateMode::Continuous | UpdateMode::Reactive { .. } => true,
                        UpdateMode::ReactiveLowPower { .. } => {
                            tao_state.low_power_event
                                || tao_state.redraw_request_sent
                                || tao_state.timeout_reached
                        }
                    }
                } else {
                    false
                };

                if update {
                    tao_state.last_update = Instant::now();
                    app.update();
                }
            }
            Event::RedrawEventsCleared => {
                {
                    // Fetch from world
                    let (tao_config, window_focused_query) = focused_window_state.get(&app.world);

                    // True if _any_ windows are currently being focused
                    let app_focused = window_focused_query.iter().any(|window| window.focused);

                    let now = Instant::now();
                    use UpdateMode::*;
                    *control_flow = match tao_config.update_mode(app_focused) {
                        Continuous => ControlFlow::Poll,
                        Reactive { max_wait } | ReactiveLowPower { max_wait } => {
                            if let Some(instant) = now.checked_add(*max_wait) {
                                ControlFlow::WaitUntil(instant)
                            } else {
                                ControlFlow::Wait
                            }
                        }
                    };
                }

                // This block needs to run after `app.update()` in `MainEventsCleared`. Otherwise,
                // we won't be able to see redraw requests until the next event, defeating the
                // purpose of a redraw request!
                let mut redraw = false;
                if let Some(app_redraw_events) = app.world.get_resource::<Events<RequestRedraw>>() {
                    if redraw_event_reader.iter(app_redraw_events).last().is_some() {
                        *control_flow = ControlFlow::Poll;
                        redraw = true;
                    }
                }

                tao_state.redraw_request_sent = redraw;
            }

            _ => (),
        }

        if tao_state.active {
            #[cfg(not(target_arch = "wasm32"))]
            let (
                commands,
                mut new_windows,
                created_window_writer,
                tao_windows,
                // adapters,
                // handlers,
                // accessibility_requested,
            ) = create_window_system_state.get_mut(&mut app.world);

            // TODO
            // #[cfg(target_arch = "wasm32")]
            // let (
            //     commands,
            //     mut new_windows,
            //     created_window_writer,
            //     tao_windows,
            //     adapters,
            //     handlers,
            //     accessibility_requested,
            //     canvas_parent_resize_channel,
            // ) = create_window_system_state.get_mut(&mut app.world);

            // Responsible for creating new windows
            create_window(
                commands,
                event_loop,
                new_windows.iter_mut(),
                created_window_writer,
                tao_windows,
                // adapters,
                // handlers,
                // accessibility_requested,
                // #[cfg(target_arch = "wasm32")]
                // canvas_parent_resize_channel,
            );

            create_window_system_state.apply(&mut app.world);
        }
    };

    // If true, returns control from Tao back to the main Bevy loop
    if return_from_run {
        run_return(&mut event_loop, event_handler);
    } else {
        run(event_loop, event_handler);
    }
}

/// Tao events that emit from UI side
#[derive(Debug)]
pub enum UiEvent<'a>
// <UiAction: Debug, AsyncAction>
{
    /// UI events regards window manipulation
    WindowEvent(WindowEvent<'a>),
    // /// User defined UiAction coming from Ui
    // UiAction(UiAction),
    // /// KeyboardEvent which dispatched from `window.document`. Make sure to pass `keyboard_event:
    // /// true` to `DioxusSettings`.
    // KeyboardEvent(KeyboardEvent),
    // /// User defined AsyncAction
    // AsyncAction(AsyncAction),
}
