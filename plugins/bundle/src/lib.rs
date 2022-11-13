// mod config;
mod schedule;
mod tool;

use bevy::{
    app::{App, Plugin},
    ecs::event::{EventReader, EventWriter},
};
// pub use config::BundleConfigPlugin;
pub use schedule::{BundleSchedulePlugin, BundleStage};
use std::{fmt::Debug, marker::PhantomData, path::PathBuf};
use tool::{InstallTools, ToolPlugin};

pub struct BundlePlugin<Config> {
    config: PhantomData<Config>,
}

impl<Config> BundlePlugin<Config>
where
    Config: Debug,
{
    pub fn new() -> Self {
        Self {
            config: PhantomData,
        }
    }
}

impl<Config> Plugin for BundlePlugin<Config>
where
    Config: 'static + Send + Sync + Debug,
{
    fn build(&self, app: &mut App) {
        app.add_plugin(BundleSchedulePlugin)
            .add_event::<ApplyBundle>()
            .add_event::<CleanBundle>()
            // .add_plugin(BundleConfigPlugin)
            .add_plugin(ToolPlugin::<Config>::new())
            .add_system_to_stage(BundleStage::First, apply_bundle);
    }
}

// Events

#[derive(Clone)]
pub struct ApplyBundle {
    pub path: PathBuf,
}

#[derive(Clone)]
pub struct CleanBundle {
    pub path: PathBuf,
}

fn apply_bundle(
    mut events: EventReader<ApplyBundle>,
    mut install_tools: EventWriter<InstallTools>,
) {
    events.iter().for_each(|e| {
        install_tools.send(InstallTools {
            path: e.path.clone(),
        });
    });
}
