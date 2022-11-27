//! Shared resources across platforms

pub mod schedule;
pub mod ui_state;

pub use dip_config as config;
pub use dip_task as task;

pub mod prelude {
    pub use crate::{
        config::{ConfigPlugin, ConfigStartupStage},
        schedule::{DipStage, DipStartupStage, UiSchedulePlugin},
        task::prelude::*,
        ui_state::{NoRootProps, NoUiAction, NoUiState, UiStateHandler},
    };
}
