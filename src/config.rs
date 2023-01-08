use dip::{
    bevy::app::{App, Plugin},
    builder::BuilderConfig,
    bundle::BundleConfig,
    core::config::SimpleConfig,
};
use serde::Deserialize;

pub struct DipConfigPlugin;

impl Plugin for DipConfigPlugin {
    fn build(&self, app: &mut App) {
        let config = SimpleConfig::new();

        app.insert_resource(
            // Provide config struct via type parameter
            config.build::<UserConfig>(
                // Path to a user defined config file
                &dirs::data_dir()
                    .unwrap()
                    .join(env!("CARGO_BIN_NAME"))
                    .join("User.toml"),
                // Default file to be included in binary
                include_str!("config/User.toml"),
            ),
        )
        .insert_resource(config.build::<ProjectConfig>(
            &std::env::current_dir().unwrap().join("Dip.toml"),
            include_str!("config/Project.toml"),
        ));
    }
}

#[derive(Deserialize, Clone)]
pub struct UserConfig {
    pub bundle: BundleConfig,
}

#[derive(Deserialize, Clone)]
pub struct ProjectConfig {
    pub builder: BuilderConfig,
}
