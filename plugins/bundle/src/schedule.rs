use bevy::{
    app::{App, Plugin},
    ecs::schedule::{StageLabel, SystemStage},
};
use dip_core::schedule::DipStage;

pub struct BundleSchedulePlugin;

impl Plugin for BundleSchedulePlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            DipStage::Prepare,
            BundleStage::Prepare,
            SystemStage::parallel(),
        )
        .add_stage_after(
            DipStage::Apply,
            BundleStage::Install,
            SystemStage::parallel(),
        )
        .add_stage_after(
            BundleStage::Install,
            BundleStage::Apply,
            SystemStage::parallel(),
        );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum BundleStage {
    Prepare,
    Install,
    Apply,
}
