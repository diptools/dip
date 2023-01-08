use bevy::{
    app::{App, Plugin},
    ecs::schedule::{StageLabel, SystemStage},
};
use dip_core::schedule::DipStage;

pub struct BuilderSchedulePlugin;

impl Plugin for BuilderSchedulePlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            DipStage::Render,
            BuilderStage::First,
            SystemStage::parallel(),
        )
        .add_stage_after(
            BuilderStage::First,
            BuilderStage::PreBuild,
            SystemStage::parallel(),
        )
        .add_stage_after(
            BuilderStage::First,
            BuilderStage::BuildApp,
            SystemStage::parallel(),
        )
        .add_stage_after(
            BuilderStage::BuildApp,
            BuilderStage::Last,
            SystemStage::parallel(),
        );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum BuilderStage {
    First,
    PreBuild,
    BuildApp,
    Last,
}
