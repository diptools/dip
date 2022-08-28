#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub use bevy_dioxus_core as core;
#[cfg(feature = "desktop")]
pub use bevy_dioxus_desktop as desktop;
pub use bevy_dioxus_macro as macros;

pub use bevy;
pub use dioxus;
