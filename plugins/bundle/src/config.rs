use bevy::{
    app::{App, Plugin},
    ecs::{schedule::ParallelSystemDescriptorCoercion, system::ResMut},
};
use config::{
    builder::{ConfigBuilder, DefaultState},
    File,
};
use dip_core::config::{build_config, ConfigPlugin as ConfigPluginRaw};
use serde::Deserialize;

pub struct BundleConfigPlugin;

impl Plugin for BundleConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ConfigPluginRaw::<BundleConfig>::with_default_str(
            include_str!("config/default.toml"),
        ))
        .add_startup_system(add_sources.before(build_config::<BundleConfig>));
    }
}

fn add_sources(mut builder: ResMut<ConfigBuilder<DefaultState>>) {
    let dip_global_config_path = dirs::data_dir().unwrap().join("dip");
    let user_bundle_config_path = dip_global_config_path.join("bundle.toml");

    match user_bundle_config_path.canonicalize() {
        Ok(user_bundle_config_path) => {
            *builder = builder.clone().add_source(File::with_name(
                &user_bundle_config_path.display().to_string(),
            ));
        }
        Err(_e) => {
            eprintln!(
                "Cannot find dip config file: {}",
                user_bundle_config_path.display()
            );
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct BundleConfig {
    pub vm: VMConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct VMConfig {
    pub tailwindcss: VersionList,
    pub nodejs: VersionList,
}

pub type VersionList = Vec<String>;
