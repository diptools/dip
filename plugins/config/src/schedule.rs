use bevy::{
    app::{App, Plugin, StartupStage},
    ecs::schedule::{StageLabel, SystemStage},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum ConfigStartupStage {
    Setup,
    Build,
    Parse,
    Merge,
}

pub struct ConfigSchedulePlugin;

impl Plugin for ConfigSchedulePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage_after(
            StartupStage::PreStartup,
            ConfigStartupStage::Setup,
            SystemStage::parallel(),
        )
        .add_startup_stage_after(
            ConfigStartupStage::Setup,
            ConfigStartupStage::Build,
            SystemStage::parallel(),
        )
        .add_startup_stage_after(
            ConfigStartupStage::Build,
            ConfigStartupStage::Parse,
            SystemStage::parallel(),
        )
        .add_startup_stage_after(
            StartupStage::Startup,
            ConfigStartupStage::Merge,
            SystemStage::parallel(),
        );
    }
}
