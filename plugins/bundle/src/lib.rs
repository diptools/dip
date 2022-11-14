mod config;
mod schedule;
mod tool;

use crate::{
    config::BundleConfigPlugin,
    schedule::{BundleSchedulePlugin, BundleStage},
    tool::{InstallTools, ToolPlugin},
};
use bevy::{
    app::{App, Plugin},
    ecs::event::{EventReader, EventWriter},
};
use std::path::PathBuf;

pub struct BundlePlugin;

impl BundlePlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for BundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BundleSchedulePlugin)
            .add_event::<ApplyBundle>()
            .add_event::<CleanBundle>()
            .add_plugin(BundleConfigPlugin)
            .add_plugin(ToolPlugin::new())
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
