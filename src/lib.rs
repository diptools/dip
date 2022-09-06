#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub use dip_cli as cli;
pub use dip_core as core;
pub use dip_macro as macros;

#[cfg(feature = "desktop")]
pub use dip_desktop as desktop;

pub use bevy;
pub use dioxus;

///
pub mod prelude {
    pub use bevy::prelude::*;
    pub use dioxus::prelude::*;
    pub use dip_core::prelude::*;
    pub use dip_macro::*;

    #[cfg(feature = "desktop")]
    pub use dip_desktop::prelude::*;
}
