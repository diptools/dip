//! UiStage for Bevy
use bevy::{
    app::{App, CoreStage, Plugin},
    ecs::schedule::{StageLabel, SystemStage},
};

/// The names of the default [`Ui`] stages.
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum UiStage {
    /// Place to register Ui event
    Action,
    /// Stage to query spawned component. Mainly for registering global state changes.
    Prepare,
    /// Apply state changes -> rerender
    Render,
}

pub struct UiSchedulePlugin;

impl Plugin for UiSchedulePlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::PreUpdate,
            UiStage::Action,
            SystemStage::parallel(),
        )
        .add_stage_after(
            CoreStage::PostUpdate,
            UiStage::Prepare,
            SystemStage::parallel(),
        )
        .add_stage_after(UiStage::Prepare, UiStage::Render, SystemStage::parallel());
    }
}
