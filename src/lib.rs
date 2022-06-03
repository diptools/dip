#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "desktop")]
pub use bevy_dioxus_desktop as desktop;
