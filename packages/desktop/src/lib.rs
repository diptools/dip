//! For build desktop application

#![deny(missing_docs)]

mod context;
mod converter;
pub mod event;
pub mod hooks;
pub mod plugin;
mod protocol;
mod runner;
pub mod setting;
pub mod stage;
mod window;

pub mod prelude {
    //! This module includes plugin, settings, events, and hooks.

    pub use crate::{
        event::*,
        hooks::*,
        plugin::DioxusPlugin,
        setting::{DioxusSettings, UpdateMode},
        stage::UiStage,
    };
    pub use bevy_dioxus_core::prelude::*;
    pub use futures_intrusive::channel::{shared::Sender, TrySendError};
}
