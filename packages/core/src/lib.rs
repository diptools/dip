//! Shared resources across platforms

pub mod schedule;
pub mod ui_state;

pub mod prelude {
    pub use crate::{
        schedule::{UiSchedulePlugin, UiStage},
        ui_state::{NoRootProps, NoUiAction, NoUiState, UiStateHandler},
    };
}
