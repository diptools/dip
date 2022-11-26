mod config;
mod platform;
mod schedule;
mod tool;

pub use crate::{
    config::{BundleConfig, BundleConfigPlugin},
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
pub struct ApplyBundle;

#[derive(Clone)]
pub struct CleanBundle;

fn apply_bundle(
    mut events: EventReader<ApplyBundle>,
    mut install_tools: EventWriter<InstallTools>,
) {
    events.iter().for_each(|_e| {
        install_tools.send(InstallTools);
    });
}

pub trait Bundler {
    fn key() -> &'static str;

    fn name() -> &'static str;

    fn bundle_dir(&self) -> &PathBuf;

    fn bundle_exists(&self) -> bool {
        self.bundle_dir().is_dir()
    }
}
