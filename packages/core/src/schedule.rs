//! UiStage for Bevy
use bevy::{
    app::{App, CoreStage, Plugin},
    ecs::schedule::{StageLabel, SystemStage},
};

/// The names of the default [`Ui`] stages.
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum UiStage {
    /// Place to write Ui event
    Action,
    /// Stage to query spawned component. Use this stage to add system requires to wait 1
    /// frame delay.
    Prepare,
    /// Apply ui state changes
    Apply,
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
        .add_stage_after(UiStage::Prepare, UiStage::Apply, SystemStage::parallel())
        .add_stage_after(UiStage::Prepare, UiStage::Render, SystemStage::parallel());
    }
}
