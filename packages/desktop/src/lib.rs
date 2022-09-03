//! For build desktop application

#![deny(missing_docs)]

pub use tokio;

mod context;
mod converter;
pub mod event;
mod event_loop;
pub mod hooks;
pub mod plugin;
mod protocol;
pub mod setting;
mod system;
mod virtual_dom;
mod window;

pub mod prelude {
    //! This module includes plugin, settings, events, and hooks.

    pub use crate::{
        event::*,
        hooks::*,
        plugin::DioxusPlugin,
        setting::{DioxusSettings, UpdateMode},
    };
    pub use bevy_dioxus_core::prelude::*;
}
