//! Shared resources across platforms

pub mod schedule;
pub mod ui_state;

pub use dip_config as config;
#[cfg(not(target_arch = "wasm32"))]
pub use dip_task as task;

pub mod prelude {
    pub use crate::{
        config::{ConfigPlugin, ConfigStartupStage},
        schedule::{DipStage, DipStartupStage, UiSchedulePlugin},
        ui_state::{NoRootProps, NoUiAction, NoUiState, UiStateHandler},
    };

    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::task::prelude::*;
}
