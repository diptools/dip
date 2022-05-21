use crate::event::{
    UiEvent::{self, *},
    WindowEvent::*,
};
use futures_intrusive::channel::shared::{Receiver, Sender};
use std::fmt::Debug;
use wry::application::event_loop::EventLoopProxy;

pub type ProxyType<CoreCommand> = EventLoopProxy<UiEvent<CoreCommand>>;

#[derive(Clone)]
pub struct UiContext<CoreCommand: Debug + 'static + Clone, UiCommand: 'static + Clone> {
    proxy: ProxyType<CoreCommand>,
    channel: (Sender<CoreCommand>, Receiver<UiCommand>),
}

impl<CoreCommand, UiCommand> UiContext<CoreCommand, UiCommand>
where
    CoreCommand: Debug + Clone,
    UiCommand: Debug + Clone,
{
    pub fn new(
        proxy: ProxyType<CoreCommand>,
        channel: (Sender<CoreCommand>, Receiver<UiCommand>),
    ) -> Self {
        Self { proxy, channel }
    }

    pub fn receiver(&self) -> Receiver<UiCommand> {
        self.channel.1.clone()
    }

    pub fn send(&self, cmd: CoreCommand) {
        self.channel
            .0
            .try_send(cmd)
            .expect("Failed to send CoreCommand");
    }

    pub fn drag(&self) {
        let _ = self.proxy.send_event(WindowEvent(DragWindow));
    }

    pub fn set_minimized(&self, minimized: bool) {
        let _ = self.proxy.send_event(WindowEvent(Minimize(minimized)));
    }

    pub fn set_maximized(&self, maximized: bool) {
        let _ = self.proxy.send_event(WindowEvent(Maximize(maximized)));
    }

    pub fn toggle_maximized(&self) {
        let _ = self.proxy.send_event(WindowEvent(MaximizeToggle));
    }

    pub fn set_visible(&self, visible: bool) {
        let _ = self.proxy.send_event(WindowEvent(Visible(visible)));
    }

    pub fn close(&self) {
        let _ = self.proxy.send_event(WindowEvent(CloseWindow));
    }

    pub fn focus(&self) {
        let _ = self.proxy.send_event(WindowEvent(FocusWindow));
    }

    pub fn set_fullscreen(&self, fullscreen: bool) {
        let _ = self.proxy.send_event(WindowEvent(Fullscreen(fullscreen)));
    }

    pub fn set_resizable(&self, resizable: bool) {
        let _ = self.proxy.send_event(WindowEvent(Resizable(resizable)));
    }

    pub fn set_always_on_top(&self, top: bool) {
        let _ = self.proxy.send_event(WindowEvent(AlwaysOnTop(top)));
    }

    pub fn set_cursor_visible(&self, visible: bool) {
        let _ = self.proxy.send_event(WindowEvent(CursorVisible(visible)));
    }

    pub fn set_cursor_grab(&self, grab: bool) {
        let _ = self.proxy.send_event(WindowEvent(CursorGrab(grab)));
    }

    pub fn set_title(&self, title: &str) {
        let _ = self
            .proxy
            .send_event(WindowEvent(SetTitle(String::from(title))));
    }

    pub fn set_decorations(&self, decoration: bool) {
        let _ = self
            .proxy
            .send_event(WindowEvent(SetDecorations(decoration)));
    }

    pub fn devtool(&self) {
        let _ = self.proxy.send_event(WindowEvent(DevTool));
    }

    pub fn eval(&self, script: impl std::string::ToString) {
        let _ = self.proxy.send_event(WindowEvent(Eval(script.to_string())));
    }
}
