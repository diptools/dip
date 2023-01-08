use dip::{
    bevy::app::{App, Plugin},
    builder::BuilderConfig,
    bundle::BundleConfig,
    core::config::{ConfigPlugin, Configurable},
};
use serde::Deserialize;
use std::any::Any;

pub struct DipConfigPlugin;

impl Plugin for DipConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(
            ConfigPlugin::new()
                .add::<UserConfig>(
                    &dirs::data_dir()
                        .unwrap()
                        .join(env!("CARGO_BIN_NAME"))
                        .join("User.toml"),
                    include_str!("config/User.toml"),
                )
                .add::<ProjectConfig>(
                    &std::env::current_dir().unwrap().join("Dip.toml"),
                    include_str!("config/Project.toml"),
                ),
        );
    }
}

#[derive(Deserialize, Clone, Any)]
pub struct UserConfig {
    pub bundle: BundleConfig,
}

#[derive(Deserialize, Clone, Any)]
pub struct ProjectConfig {
    pub builder: BuilderConfig,
}

impl Configurable for ProjectConfig {}
