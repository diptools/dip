// mod config;
mod schedule;
mod tool;

use bevy::{
    app::{App, Plugin},
    ecs::event::{EventReader, EventWriter},
};
// pub use config::BundleConfigPlugin;
pub use schedule::{BundleSchedulePlugin, BundleStage};
use std::path::PathBuf;
use tool::{ApplyTools, InstallTools, ToolPlugin};

pub struct BundlePlugin;

impl Plugin for BundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BundleSchedulePlugin)
            .add_event::<ApplyBundle>()
            .add_event::<BundleApplied>()
            // .add_plugin(BundleConfigPlugin)
            .add_plugin(ToolPlugin)
            .add_system_to_stage(BundleStage::Prepare, apply_bundle);
    }
}

// Events

pub struct ApplyBundle {
    pub verbose: bool,
    pub path: PathBuf,
}

pub struct BundleApplied;

fn apply_bundle(
    mut events: EventReader<ApplyBundle>,
    mut install_tools: EventWriter<InstallTools>,
    mut apply_tools: EventWriter<ApplyTools>,
) {
    events
        .iter()
        .map(|e| {
            (
                InstallTools {
                    verbose: e.verbose,
                    path: e.path.clone(),
                },
                ApplyTools {
                    verbose: e.verbose,
                    path: e.path.clone(),
                },
            )
        })
        .for_each(|(install, apply)| {
            install_tools.send(install);
            apply_tools.send(apply);
        });
}
