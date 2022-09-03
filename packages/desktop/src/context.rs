use crate::event::{
    UiEvent::{self, *},
    WindowEvent::*,
};
use std::fmt::Debug;
use tokio::sync::mpsc::Sender;
use wry::application::event_loop::EventLoopProxy;

pub type ProxyType<UiAction> = EventLoopProxy<UiEvent<UiAction>>;

#[derive(Clone)]
pub struct UiContext<UiAction: Debug + 'static + Clone> {
    proxy: ProxyType<UiAction>,
    ui_action_tx: Sender<UiAction>,
}

impl<UiAction> UiContext<UiAction>
where
    UiAction: Debug + Clone,
{
    pub fn new(proxy: ProxyType<UiAction>, ui_action_tx: Sender<UiAction>) -> Self {
        Self {
            proxy,
            ui_action_tx,
        }
    }

    pub fn send(&self, action: UiAction) {
        self.ui_action_tx
            .try_send(action)
            .expect("Failed to send UiAction");
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

    pub fn rerender(&self) {
        self.proxy
            .send_event(UiEvent::WindowEvent(Rerender))
            .unwrap();
    }
}
