//! DipStage for Bevy
use bevy::{
    app::{App, CoreStage, Plugin, StartupStage},
    ecs::schedule::{StageLabel, SystemStage},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum DipStartupStage {
    /// Place to send cli subcommand
    Action,
}

/// The names of the default [`Ui`] stages.
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum DipStage {
    /// Place to send Ui event or cli subcommand
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
        app.add_startup_stage_after(
            StartupStage::PreStartup,
            DipStartupStage::Action,
            SystemStage::parallel(),
        )
        .add_stage_after(
            CoreStage::PreUpdate,
            DipStage::Action,
            SystemStage::parallel(),
        )
        .add_stage_after(
            CoreStage::PostUpdate,
            DipStage::Prepare,
            SystemStage::parallel(),
        )
        .add_stage_after(DipStage::Prepare, DipStage::Apply, SystemStage::parallel())
        .add_stage_after(DipStage::Prepare, DipStage::Render, SystemStage::parallel());
    }
}
