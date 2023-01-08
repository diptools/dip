mod bundler;
mod config;
mod installer;
mod platform;
mod schedule;
mod tool;

use bevy::app::{App, Plugin};

pub use crate::{
    bundler::Bundler,
    config::BundleConfig,
    installer::Installer,
    schedule::{BundleSchedulePlugin, BundleStage},
};

#[cfg(target_family = "unix")]
pub use tool::UnixToolPlugin;

pub struct BundlePlugin;

impl Plugin for BundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BundleSchedulePlugin)
            .add_event::<ApplyBundle>()
            .add_event::<CleanBundle>();

        #[cfg(target_family = "unix")]
        app.add_plugin(UnixToolPlugin);
    }
}

#[derive(Clone)]
pub struct ApplyBundle;

#[derive(Clone)]
pub struct CleanBundle;
