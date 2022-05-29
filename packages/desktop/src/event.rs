use crate::converter;
use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    window::{ReceivedCharacter, WindowId},
};
use dioxus_core::{ElementId, EventPriority, UserEvent};
use serde::Deserialize;
use serde_json::Value;
use serde_repr::*;
use std::fmt::Debug;

#[derive(Debug)]
pub enum UiEvent<CoreCommand: Debug> {
    WindowEvent(WindowEvent),
    CoreCommand(CoreCommand),
    KeyboardEvent(KeyboardEvent),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct IpcMessage {
    method: String,
    params: serde_json::Value,
}

impl IpcMessage {
    pub fn method(&self) -> &str {
        self.method.as_str()
    }

    pub fn params(self) -> serde_json::Value {
        self.params
    }

    pub fn from_payload(payload: &str) -> Option<IpcMessage> {
        match serde_json::from_str(payload) {
            Ok(message) => Some(message),
            Err(e) => {
                log::error!("could not parse IPC message, error: {}", e);
                None
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ImEvent {
    event: String,
    mounted_dom_id: u64,
    contents: serde_json::Value,
}

pub fn trigger_from_serialized(val: serde_json::Value) -> UserEvent {
    let ImEvent {
        event,
        mounted_dom_id,
        contents,
    } = serde_json::from_value(val).unwrap();

    let mounted_dom_id = Some(ElementId(mounted_dom_id as usize));

    let name = converter::convert_event_type_to_name(&event);
    let event = converter::convert_synthetic_event(&event, contents);

    UserEvent {
        name,
        priority: EventPriority::Low,
        scope_id: None,
        element: mounted_dom_id,
        data: event,
    }
}

#[derive(Debug)]
pub enum WindowEvent {
    Update,

    CloseWindow,
    DragWindow,
    FocusWindow,

    Visible(bool),
    Minimize(bool),
    Maximize(bool),
    MaximizeToggle,
    Resizable(bool),
    AlwaysOnTop(bool),
    Fullscreen(bool),

    CursorVisible(bool),
    CursorGrab(bool),

    SetTitle(String),
    SetDecorations(bool),

    SetZoomLevel(f64),

    Print,
    DevTool,

    Eval(String),
}

#[derive(Debug, Clone)]
pub struct UpdateDom;

#[derive(Debug, Clone)]
pub struct DomUpdated {
    pub id: WindowId,
}

#[derive(Debug, Clone)]
pub struct WindowDragged {
    pub id: WindowId,
}

#[derive(Debug, Clone)]
pub struct WindowMinimized {
    pub id: WindowId,
    pub minimized: bool,
}

#[derive(Debug, Clone)]
pub struct WindowMaximized {
    pub id: WindowId,
    pub maximized: bool,
}

#[derive(Debug, Clone)]
pub struct MaximizeToggled {
    pub id: WindowId,
    pub maximized: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum KeyboardEvent {
    #[serde(rename = "keydown")]
    Keydown {
        key: String,
        #[serde(rename = "key_code")]
        scan_code: u32,
        location: Location,
    },
    #[serde(rename = "keyup")]
    Keyup {
        key: String,
        #[serde(rename = "key_code")]
        scan_code: u32,
        location: Location,
    },
}

impl KeyboardEvent {
    pub fn from_value(value: Value) -> KeyboardEvent {
        serde_json::from_value(value).unwrap()
    }

    pub fn to_input(&self) -> KeyboardInput {
        match self {
            KeyboardEvent::Keydown {
                key,
                scan_code,
                location,
            } => KeyboardInput {
                scan_code: *scan_code,
                key_code: converter::try_convert_key_code(key, location),
                state: ElementState::Pressed,
            },
            KeyboardEvent::Keyup {
                key,
                scan_code,
                location,
            } => KeyboardInput {
                scan_code: *scan_code,
                key_code: converter::try_convert_key_code(key, location),
                state: ElementState::Released,
            },
        }
    }

    pub fn try_to_char(&self) -> Option<ReceivedCharacter> {
        let id = WindowId::primary();

        match self.key() {
            "Enter" => Some(ReceivedCharacter { id, char: '\r' }),
            "Backspace" => Some(ReceivedCharacter { id, char: '\u{7f}' }),
            key if key.len() > 1 => None,
            _ => Some(ReceivedCharacter {
                id,
                char: self.key().chars().next().unwrap(),
            }),
        }
    }

    pub fn key(&self) -> &str {
        match self {
            KeyboardEvent::Keyup { key, .. } | KeyboardEvent::Keydown { key, .. } => key,
        }
    }
}

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum Location {
    Standard,
    Left,
    Right,
    Numpad,
    Mobile,
    Joystick,
}
