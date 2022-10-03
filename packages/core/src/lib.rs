//! Shared resources across platforms

pub mod schedule;
pub mod ui_state;

pub use dip_task as task;

pub mod prelude {
    pub use crate::{
        schedule::{UiSchedulePlugin, UiStage},
        ui_state::{NoRootProps, NoUiAction, NoUiState, UiStateHandler},
    };
    pub use dip_task::AsyncActionPool;
}
