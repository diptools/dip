use crate::{converter, window::DioxusWindows};
use bevy::{
    ecs::{
        event::EventWriter,
        system::{NonSendMut, ResMut},
    },
    log::{error, warn},
    math::{UVec2, Vec2},
    window::{WindowClosed, WindowCommand, WindowMode, WindowScaleFactorChanged, Windows},
};
use wry::application::{
    dpi::{LogicalPosition, LogicalSize, PhysicalPosition},
    window::Fullscreen,
};

pub fn change_window(
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
