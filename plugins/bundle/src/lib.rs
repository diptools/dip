#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

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
<<<<<<< HEAD
pub struct ApplyBundle {
    pub path: PathBuf,
}

#[derive(Clone)]
pub struct CleanBundle {
    pub path: PathBuf,
}
=======
pub struct ApplyBundle;

#[derive(Clone)]
pub struct CleanBundle;
>>>>>>> e04d1b0 (Merge bundle config with cli arguments)

fn apply_bundle(
    mut events: EventReader<ApplyBundle>,
    mut install_tools: EventWriter<InstallTools>,
) {
<<<<<<< HEAD
    events.iter().for_each(|e| {
        install_tools.send(InstallTools {
            path: e.path.clone(),
        });
=======
    events.iter().for_each(|_e| {
        install_tools.send(InstallTools);
>>>>>>> e04d1b0 (Merge bundle config with cli arguments)
    });
}

pub trait Bundler {
    fn name() -> &'static str;

    fn bundle(&self) -> &PathBuf;

    fn bundle_exists(&self) -> bool {
        self.bundle().is_dir()
    }
}
