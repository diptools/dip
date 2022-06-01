#![warn(missing_docs)]
//! Build desktop app

mod context;
mod converter;
mod event;
mod hooks;
mod plugin;
mod protocol;
mod runner;
mod setting;
mod window;

/// This module includes plugin, settings, events, and hooks.
pub mod prelude {
    pub use crate::{
        event::*,
        hooks::*,
        plugin::DioxusPlugin,
        setting::{DioxusSettings, UpdateMode},
    };
}
