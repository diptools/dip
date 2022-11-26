use bevy::{
    app::{App, Plugin},
    ecs::schedule::{StageLabel, SystemStage},
};
use dip_core::schedule::DipStage;

pub struct BundleSchedulePlugin;

impl Plugin for BundleSchedulePlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            DipStage::Render,
            BundleStage::First,
            SystemStage::parallel(),
        )
        .add_stage_after(
            BundleStage::First,
            BundleStage::PreScript,
            SystemStage::parallel(),
        )
        .add_stage_after(
            BundleStage::PreScript,
            BundleStage::Clean,
            SystemStage::parallel(),
        )
        .add_stage_after(
            BundleStage::Clean,
            BundleStage::Install,
            SystemStage::parallel(),
        )
        .add_stage_after(
            BundleStage::Install,
            BundleStage::Apply,
            SystemStage::parallel(),
        )
        .add_stage_after(
            BundleStage::Apply,
            BundleStage::PostScript,
            SystemStage::parallel(),
        )
        .add_stage_before(
            BundleStage::PostScript,
            BundleStage::Last,
            SystemStage::parallel(),
        );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum BundleStage {
    First,
    PreScript,
    Clean,
    Install,
    Apply,
    PostScript,
    Last,
}
