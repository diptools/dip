#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub use dip_core as core;

#[cfg(feature = "cli")]
pub use dip_cli as cli;

#[cfg(feature = "desktop")]
pub use dip_desktop as desktop;

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
}
