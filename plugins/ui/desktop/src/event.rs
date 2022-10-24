//! Includes events coming from UI to ECS runtime and to communicate bewtween systems

use crate::converter;
use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    window::{ReceivedCharacter, WindowId},
};
use dioxus_core::{ElementId, EventPriority, UserEvent};
use serde::Deserialize;
use serde_json::Value;
use serde_repr::*;
use std::fmt::Debug;

/// Tao events that emit from UI side
#[derive(Debug)]
pub enum UiEvent<UiAction: Debug, AsyncAction> {
    /// UI events regards window manipulation
    WindowEvent(WindowEvent),
    /// User defined UiAction coming from Ui
    UiAction(UiAction),
    /// KeyboardEvent which dispatched from `window.document`. Make sure to pass `keyboard_event:
    /// true` to `DioxusSettings`.
    KeyboardEvent(KeyboardEvent),
    /// User defined AsyncAction
    AsyncAction(AsyncAction),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct IpcMessage {
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

pub(crate) fn trigger_from_serialized(val: serde_json::Value) -> UserEvent {
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

/// Dioxus specific window events
#[derive(Debug)]
pub enum WindowEvent {
    /// When VirtualDOM applies all edits
    Rerender,

    /// When close window is requested
    CloseWindow,
    /// When drag window is requested
    DragWindow,
    /// When window is focused
    FocusWindow,

    /// Event to change window visibility
    Visible(bool),
    /// Event to minimuze window size
    Minimize(bool),
    /// Event to maximize window size
    Maximize(bool),
    /// Event to toggle between normal and maximized window
    MaximizeToggle,
    /// Event to change resizable
    Resizable(bool),
    /// Event to bring window to the top most always
    AlwaysOnTop(bool),
    /// Event to enter fullscreen mode
    Fullscreen(bool),

    /// Event to configure cursor visibility
    CursorVisible(bool),
    /// Event to configure cursor grab
    CursorGrab(bool),

    /// Event to set window title
    SetTitle(String),
    /// Event to show/hide window decorations
    SetDecorations(bool),

    /// Event to change window zoom level
    SetZoomLevel(f64),

    /// Event to print window
    Print,
    /// Event to open devtools
    DevTool,

    /// Event to execute JavaScript
    Eval(String),
}

/// Rust representation of web KeyboardEvent
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum KeyboardEvent {
    /// Rust representation of keydown event
    #[serde(rename = "keydown")]
    Keydown {
        /// key string
        key: String,
        /// scan code
        #[serde(rename = "key_code")]
        scan_code: u32,
        /// location
        location: Location,
    },
    /// Rust representation of keyup event
    #[serde(rename = "keyup")]
    Keyup {
        /// key string
        key: String,
        /// scan code
        #[serde(rename = "key_code")]
        scan_code: u32,
        /// location
        location: Location,
    },
}

impl KeyboardEvent {
    pub(crate) fn from_value(value: Value) -> KeyboardEvent {
        serde_json::from_value(value).unwrap()
    }

    /// Convert into KeyboardInput type from Bevy
    pub fn to_input(&self) -> KeyboardInput {
        match self {
            KeyboardEvent::Keydown {
                key,
                scan_code,
                location,
            } => KeyboardInput {
                scan_code: *scan_code,
                key_code: converter::try_convert_key_code(key, location),
                state: ButtonState::Pressed,
            },
            KeyboardEvent::Keyup {
                key,
                scan_code,
                location,
            } => KeyboardInput {
                scan_code: *scan_code,
                key_code: converter::try_convert_key_code(key, location),
                state: ButtonState::Released,
            },
        }
    }

    pub(crate) fn try_to_char(&self) -> Option<ReceivedCharacter> {
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

    /// Get key without destructuring both KeyDown and KeyUp event
    pub fn key(&self) -> &str {
        match self {
            KeyboardEvent::Keyup { key, .. } | KeyboardEvent::Keydown { key, .. } => key,
        }
    }
}

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(u8)]
/// Key location, more on MDN docs
pub enum Location {
    /// default
    Standard,
    /// left key
    Left,
    /// right key
    Right,
    /// numpad key
    Numpad,
    /// mobile key
    Mobile,
    /// joystick key
    Joystick,
}
