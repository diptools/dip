#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub use dip_core as core;

#[cfg(feature = "cli")]
pub use dip_cli as cli;

#[cfg(feature = "desktop")]
pub use dip_desktop as desktop;

#[cfg(feature = "web")]
pub use dip_web as web;

#[cfg(not(target_arch = "wasm32"))]
pub use dip_bundle as bundle;

#[cfg(not(target_arch = "wasm32"))]
pub use dip_builder as builder;

/// Web3 related plugins
pub mod web3 {
    #[cfg(not(target_arch = "wasm32"))]
    pub use dip_device as device;
}

pub use bevy;

#[cfg(feature = "desktop")]
pub use dioxus;

pub use tokio;

///
pub mod prelude {
    pub use bevy::prelude::*;
    pub use dioxus::prelude::*;
    pub use dip_core::prelude::*;
    pub use dip_macro::{ui_action, ui_state, ConfigPlugin};

    #[cfg(feature = "cli")]
    pub use dip_cli::prelude::*;

    #[cfg(feature = "desktop")]
    pub use dip_desktop::prelude::*;

    #[cfg(feature = "web")]
    pub use dip_web::prelude::*;
}
