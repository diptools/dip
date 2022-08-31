use crate::{
    context::ProxyType,
    event::{trigger_from_serialized, IpcMessage, KeyboardEvent, UiEvent, WindowEvent},
    protocol,
    setting::DioxusSettings,
};
use bevy::{
    ecs::world::WorldCell,
    math::IVec2,
    utils::HashMap,
    window::{Window as BevyWindow, WindowDescriptor, WindowId, WindowMode},
};
use dioxus_core::SchedulerMsg;
use futures_channel::mpsc;
use raw_window_handle::HasRawWindowHandle;
use std::{
    fmt::{self, Debug},
    marker::PhantomData,
    sync::{atomic::AtomicBool, Arc, Mutex},
};
use wry::{
    application::{
        dpi::{LogicalPosition, LogicalSize, PhysicalPosition},
        error::ExternalError,
        event_loop::EventLoop,
        monitor::{MonitorHandle, VideoMode},
        window::{Fullscreen, Window as TaoWindow, WindowBuilder, WindowId as TaoWindowId},
    },
    webview::{WebView, WebViewBuilder},
};

#[derive(Default)]
pub struct DioxusWindows {
    windows: HashMap<TaoWindowId, Window>,
    window_id_to_tao: HashMap<WindowId, TaoWindowId>,
    tao_to_window_id: HashMap<TaoWindowId, WindowId>,
    _not_send_sync: PhantomData<*const ()>,

    quit_app_on_close: bool,
}

impl Debug for DioxusWindows {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DioxusWindows")
            .field("windonw keys", &self.windows.keys())
            .field("window_id_to_tao", &self.window_id_to_tao)
            .field("tao_to_window_id", &self.tao_to_window_id)
            .field("quit_app_on_close", &self.quit_app_on_close)
            .finish()
    }
}

impl DioxusWindows {
    pub fn get(&self, id: WindowId) -> Option<&Window> {
        self.window_id_to_tao
            .get(&id)
            .and_then(|id| self.windows.get(id))
    }

    pub fn get_mut(&mut self, id: WindowId) -> Option<&mut Window> {
        self.window_id_to_tao
            .get(&id)
            .and_then(|id| self.windows.get_mut(id))
    }

    pub fn get_tao_window(&self, id: WindowId) -> Option<&TaoWindow> {
        self.get(id).and_then(|window| Some(window.tao_window()))
    }

    pub fn get_window_id(&self, id: TaoWindowId) -> Option<WindowId> {
        self.tao_to_window_id.get(&id).cloned()
    }

    pub fn remove(&mut self, id: WindowId) -> Option<Window> {
        let tao_window_id = self.window_id_to_tao.remove(&id)?;
        self.windows.remove(&tao_window_id)
    }

    pub fn create<CoreCommand, Props>(
        &mut self,
        world: &WorldCell,
        window_id: WindowId,
        window_descriptor: &WindowDescriptor,
    ) -> BevyWindow
    where
        CoreCommand: 'static + Send + Sync + Clone + Debug,
        Props: 'static + Send + Sync + Clone,
    {
        let event_loop = world
            .get_non_send_resource_mut::<EventLoop<UiEvent<CoreCommand>>>()
            .unwrap();
        let proxy = event_loop.create_proxy();
        let dom_tx = world
            .get_resource::<mpsc::UnboundedSender<SchedulerMsg>>()
            .unwrap();
        let edit_queue = world
            .get_resource::<Arc<Mutex<Vec<String>>>>()
            .unwrap()
            .clone();

        let tao_window = Self::create_tao_window::<CoreCommand>(&event_loop, &window_descriptor);
        let tao_window_id = tao_window.id();

        let bevy_window = Self::create_bevy_window(window_id, &tao_window, &window_descriptor);
        let (webview, is_ready) = Self::create_webview::<CoreCommand, Props>(
            world,
            window_descriptor,
            tao_window,
            proxy,
            dom_tx.clone(),
        );

        self.windows.insert(
            tao_window_id,
            Window::new(webview, dom_tx.clone(), is_ready, edit_queue),
        );
        self.window_id_to_tao.insert(window_id, tao_window_id);
        self.tao_to_window_id.insert(tao_window_id, window_id);

        bevy_window
    }

    pub fn get_fitting_videomode(monitor: &MonitorHandle, width: u32, height: u32) -> VideoMode {
        let mut modes = monitor.video_modes().collect::<Vec<_>>();

        fn abs_diff(a: u32, b: u32) -> u32 {
            if a > b {
                return a - b;
            }
            b - a
        }

        modes.sort_by(|a, b| {
            use std::cmp::Ordering::*;
            match abs_diff(a.size().width, width).cmp(&abs_diff(b.size().width, width)) {
                Equal => {
                    match abs_diff(a.size().height, height).cmp(&abs_diff(b.size().height, height))
                    {
                        Equal => b.refresh_rate().cmp(&a.refresh_rate()),
                        default => default,
                    }
                }
                default => default,
            }
        });

        modes.first().unwrap().clone()
    }

    pub fn get_best_videomode(monitor: &MonitorHandle) -> VideoMode {
        let mut modes = monitor.video_modes().collect::<Vec<_>>();
        modes.sort_by(|a, b| {
            use std::cmp::Ordering::*;
            match b.size().width.cmp(&a.size().width) {
                Equal => match b.size().height.cmp(&a.size().height) {
                    Equal => b.refresh_rate().cmp(&a.refresh_rate()),
                    default => default,
                },
                default => default,
            }
        });

        modes.first().unwrap().clone()
    }

    fn create_tao_window<CoreCommand>(
        event_loop: &EventLoop<UiEvent<CoreCommand>>,
        window_descriptor: &WindowDescriptor,
    ) -> TaoWindow
    where
        CoreCommand: Debug,
    {
        let mut tao_window_builder = WindowBuilder::new().with_title(&window_descriptor.title);

        tao_window_builder = match window_descriptor.mode {
            WindowMode::BorderlessFullscreen => tao_window_builder
                .with_fullscreen(Some(Fullscreen::Borderless(event_loop.primary_monitor()))),
            WindowMode::Fullscreen => {
                tao_window_builder.with_fullscreen(Some(Fullscreen::Exclusive(
                    Self::get_best_videomode(&event_loop.primary_monitor().unwrap()),
                )))
            }
            WindowMode::SizedFullscreen => tao_window_builder.with_fullscreen(Some(
                Fullscreen::Exclusive(Self::get_fitting_videomode(
                    &event_loop.primary_monitor().unwrap(),
                    window_descriptor.width as u32,
                    window_descriptor.height as u32,
                )),
            )),
            _ => {
                let WindowDescriptor {
                    width,
                    height,
                    position,
                    scale_factor_override,
                    ..
                } = window_descriptor;

                use bevy::window::WindowPosition::*;
                match position {
                    Automatic => { /* Window manager will handle position */ }
                    Centered(monitor_selection) => {
                        use bevy::window::MonitorSelection::*;
                        let maybe_monitor = match monitor_selection {
                            Current => {
                                log::warn!("Can't select current monitor on window creation!");
                                None
                            }
                            Primary => event_loop.primary_monitor(),
                            Number(n) => event_loop.available_monitors().nth(*n),
                        };

                        if let Some(monitor) = maybe_monitor {
                            let screen_size = monitor.size();

                            let scale_factor = scale_factor_override.unwrap_or(1.0);

                            // Logical to physical window size
                            let (width, height): (u32, u32) = LogicalSize::new(*width, *height)
                                .to_physical::<u32>(scale_factor)
                                .into();

                            let position = PhysicalPosition {
                                x: screen_size.width.saturating_sub(width) as f64 / 2.
                                    + monitor.position().x as f64,
                                y: screen_size.height.saturating_sub(height) as f64 / 2.
                                    + monitor.position().y as f64,
                            };

                            tao_window_builder = tao_window_builder.with_position(position);
                        } else {
                            log::warn!("Couldn't get monitor selected with: {monitor_selection:?}");
                        }
                    }
                    At(position) => {
                        if let Some(sf) = scale_factor_override {
                            tao_window_builder = tao_window_builder.with_position(
                                LogicalPosition::new(position[0] as f64, position[1] as f64)
                                    .to_physical::<f64>(*sf),
                            );
                        } else {
                            tao_window_builder = tao_window_builder.with_position(
                                LogicalPosition::new(position[0] as f64, position[1] as f64),
                            );
                        }
                    }
                }

                if let Some(sf) = scale_factor_override {
                    tao_window_builder
                        .with_inner_size(LogicalSize::new(*width, *height).to_physical::<f64>(*sf))
                } else {
                    tao_window_builder.with_inner_size(LogicalSize::new(*width, *height))
                }
            }
            .with_resizable(window_descriptor.resizable)
            .with_decorations(window_descriptor.decorations)
            .with_transparent(window_descriptor.transparent),
        };

        let constraints = window_descriptor.resize_constraints.check_constraints();
        let min_inner_size = LogicalSize {
            width: constraints.min_width,
            height: constraints.min_height,
        };
        let max_inner_size = LogicalSize {
            width: constraints.max_width,
            height: constraints.max_height,
        };

        let tao_window_builder =
            if constraints.max_width.is_finite() && constraints.max_height.is_finite() {
                tao_window_builder
                    .with_min_inner_size(min_inner_size)
                    .with_max_inner_size(max_inner_size)
            } else {
                tao_window_builder.with_min_inner_size(min_inner_size)
            };

        let tao_window = tao_window_builder.build(&event_loop).unwrap();

        if window_descriptor.cursor_locked {
            match tao_window.set_cursor_grab(true) {
                Ok(_) => {}
                Err(ExternalError::NotSupported(_)) => {}
                Err(err) => Err(err).unwrap(),
            }
        }

        tao_window.set_cursor_visible(window_descriptor.cursor_visible);

        tao_window
    }

    fn create_bevy_window(
        window_id: WindowId,
        tao_window: &TaoWindow,
        window_descriptor: &WindowDescriptor,
    ) -> BevyWindow {
        BevyWindow::new(
            window_id,
            window_descriptor,
            tao_window.inner_size().width,
            tao_window.inner_size().height,
            tao_window.scale_factor(),
            tao_window
                .outer_position()
                .ok()
                .map(|position| IVec2::new(position.x, position.y)),
            tao_window.raw_window_handle(),
        )
    }

    fn create_webview<CoreCommand, Props>(
        world: &WorldCell,
        window_descriptor: &WindowDescriptor,
        tao_window: TaoWindow,
        proxy: ProxyType<CoreCommand>,
        dom_tx: mpsc::UnboundedSender<SchedulerMsg>,
    ) -> (WebView, Arc<AtomicBool>)
    where
        CoreCommand: 'static + Send + Sync + Clone + Debug,
        Props: 'static,
    {
        let mut settings = world
            .get_non_send_resource_mut::<DioxusSettings<Props>>()
            .unwrap();
        let is_ready = Arc::new(AtomicBool::new(false));

        let file_drop_handler = settings.file_drop_handler.take();
        let custom_head = settings.custom_head.clone();
        let resource_dir = settings.resource_dir.clone();
        let index_file = settings.custom_index.clone();
        let is_ready_clone = is_ready.clone();

        let mut webview = WebViewBuilder::new(tao_window)
            .unwrap()
            .with_transparent(window_descriptor.transparent)
            .with_url("dioxus://index.html/")
            .unwrap()
            .with_ipc_handler(move |_window: &TaoWindow, payload: String| {
                IpcMessage::from_payload(&payload)
                    .map(|message| match message.method() {
                        "user_event" => {
                            let event = trigger_from_serialized(message.params());
                            log::trace!("IpcMessage user_event: {event:?}");
                            dom_tx.unbounded_send(SchedulerMsg::Event(event)).unwrap();
                        }
                        "keyboard_event" => {
                            log::trace!("IpcMessage: keyboard_event");
                            let event = KeyboardEvent::from_value(message.params());
                            proxy.send_event(UiEvent::KeyboardEvent(event)).unwrap();
                        }
                        "initialize" => {
                            log::trace!("IpcMessage: initialize");
                            is_ready_clone.store(true, std::sync::atomic::Ordering::Relaxed);
                            let _ = proxy.send_event(UiEvent::WindowEvent(WindowEvent::Rerender));
                        }
                        "browser_open" => {
                            log::trace!("IpcMessage: browser_open");
                            let data = message.params();
                            log::trace!("Open browser: {:?}", data);
                            if let Some(temp) = data.as_object() {
                                if temp.contains_key("href") {
                                    let url = temp.get("href").unwrap().as_str().unwrap();
                                    if let Err(e) = webbrowser::open(url) {
                                        log::error!("Open Browser error: {:?}", e);
                                    }
                                }
                            }
                        }
                        _ => {}
                    })
                    .unwrap_or_else(|| {
                        log::warn!("invalid IPC message received");
                    })
            })
            .with_custom_protocol(String::from("dioxus"), move |r| {
                protocol::handler(
                    r,
                    resource_dir.clone(),
                    custom_head.clone(),
                    index_file.clone(),
                )
            })
            .with_file_drop_handler(move |window, evet| {
                file_drop_handler
                    .as_ref()
                    .map(|handler| handler(window, evet))
                    .unwrap_or_default()
            });

        for (name, handler) in settings.protocols.drain(..) {
            webview = webview.with_custom_protocol(name, handler)
        }

        if settings.disable_context_menu {
            // in release mode, we don't want to show the dev tool or reload menus
            webview = webview.with_initialization_script(
                r#"
                        if (document.addEventListener) {
                            document.addEventListener('contextmenu', function(e) {
                                alert("You've tried to open context menu");
                                e.preventDefault();
                            }, false);
                        } else {
                            document.attachEvent('oncontextmenu', function() {
                                alert("You've tried to open context menu");
                                window.event.returnValue = false;
                            });
                        }
                    "#,
            )
        } else {
            // in debug, we are okay with the reload menu showing and dev tool
            webview = webview.with_devtools(true);
        }

        if settings.keyboard_event {
            webview = webview.with_initialization_script(
                r#"
                    function serializeIpcMessage(method, params = {}) {
                      return JSON.stringify({ method, params });
                    }

                    function serialize_keyboard_event(e) {
                      let {
                        charCode,
                        key,
                        altKey,
                        ctrlKey,
                        metaKey,
                        keyCode,
                        shiftKey,
                        location,
                        repeat,
                        which,
                        type,
                      } = event;
                      return {
                        char_code: charCode,
                        key,
                        alt_key: altKey,
                        ctrl_key: ctrlKey,
                        meta_key: metaKey,
                        key_code: keyCode,
                        shift_key: shiftKey,
                        location,
                        repeat,
                        which,
                        type,
                        locale: "locale",
                      };
                    }

                    function handleKeyEvent(e) {
                      e.preventDefault();
                      window.ipc.postMessage(serializeIpcMessage("keyboard_event", serialize_keyboard_event(e)))
                    }

                    document.addEventListener('keydown', handleKeyEvent, true);
                    document.addEventListener('keyup', handleKeyEvent, true);
                "#
            );
        }

        (webview.build().unwrap(), is_ready)
    }
}

pub struct Window {
    pub webview: WebView,
    pub dom_tx: mpsc::UnboundedSender<SchedulerMsg>,
    is_ready: Arc<AtomicBool>,
    edit_queue: Arc<Mutex<Vec<String>>>,
}

impl Window {
    fn new(
        webview: WebView,
        dom_tx: mpsc::UnboundedSender<SchedulerMsg>,
        is_ready: Arc<AtomicBool>,
        edit_queue: Arc<Mutex<Vec<String>>>,
    ) -> Self {
        Self {
            webview,
            dom_tx,
            is_ready,
            edit_queue,
        }
    }

    pub fn tao_window(&self) -> &TaoWindow {
        &self.webview.window()
    }

    pub fn rerender(&mut self) {
        log::trace!("rerender: webview.evaluate_script()");
        if self.is_ready.load(std::sync::atomic::Ordering::Relaxed) {
            let mut queue = self.edit_queue.lock().unwrap();

            for edit in queue.drain(..) {
                self.webview
                    .evaluate_script(&format!("window.interpreter.handleEdits({})", edit))
                    .unwrap();
            }
        }
    }
}
