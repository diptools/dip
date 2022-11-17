use bevy::{
    app::{App, Plugin, StartupStage},
    ecs::schedule::{StageLabel, SystemStage},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum ConfigStartupStage {
    Setup,
    Build,
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
        );
    }
}
