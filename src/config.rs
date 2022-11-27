use config::{
    builder::{ConfigBuilder, DefaultState},
    File,
};
use dip::{
    bevy::{
        app::{App, Plugin},
        ecs::{schedule::ParallelSystemDescriptorCoercion, system::ResMut},
    },
    core::config::{build_config, ConfigPlugin as ConfigPluginRaw},
};
use serde::Deserialize;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ConfigPluginRaw::<DipConfig>::new())
            .add_startup_system(add_sources.before(build_config::<DipConfig>));
    }
}

fn add_sources(mut builder: ResMut<ConfigBuilder<DefaultState>>) {
    *builder = builder
        .clone()
        .add_source(File::with_name("src/config/default"));
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct DipConfig {
    bundle: Bundle,
}

#[derive(Deserialize, Debug)]
pub struct Bundle {
    vm: VersionManager,
}

#[derive(Deserialize, Debug)]
pub struct VersionManager {
    tailwindcss: VersionList,
    nodejs: VersionList,
}

pub type VersionList = Vec<String>;
