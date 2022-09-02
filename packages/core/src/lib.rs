//! Shared resources across platforms

pub mod global_state;
pub mod schedule;

pub mod prelude {
    pub use crate::{
        global_state::{EmptyGlobalState, GlobalStateHandler},
        schedule::{UiSchedulePlugin, UiStage},
    };
    pub use bevy::prelude::*;
    pub use bevy_dioxus_macro::{global_state, ui_action, ui_action_creator};
    pub use dioxus::prelude::*;
}
