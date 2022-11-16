use bevy::{
    app::{App, Plugin},
    ecs::system::ResMut,
};
use config::{
    builder::{ConfigBuilder, DefaultState},
    File,
};
use dip_core::{config::ConfigPlugin as ConfigPluginRaw, prelude::ConfigStartupStage};
use serde::Deserialize;
use std::{collections::HashSet, fs, path::PathBuf};

pub struct BundleConfigPlugin;

impl Plugin for BundleConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ConfigPluginRaw::<BundleConfig>::with_default_str(
            include_str!("config/default.toml"),
        ))
        .add_startup_system_to_stage(ConfigStartupStage::Setup, add_sources);
    }
}

fn add_sources(mut builder: ResMut<ConfigBuilder<DefaultState>>) {
    let config_file = BundleConfig::config_file();

    match config_file.canonicalize() {
        Ok(p) => {
            *builder = builder
                .clone()
                .add_source(File::with_name(&p.display().to_string()));
        }
        Err(_e) => {
            eprintln!("Cannot find dip config file: {}", config_file.display());
        }
    }
}

/// General dip configuration
// TODO: This struct is not only for bundle feature. Move to somewhere general.
pub struct Config;

impl Config {
    pub fn config_dir() -> PathBuf {
        let p = dirs::data_dir().unwrap().join("dip");
        Self::ensure_dir(&p);

        p
    }

    pub fn ensure_dir(p: &PathBuf) {
        if !&p.is_dir() {
            fs::create_dir_all(&p).unwrap();
        }
    }

    pub fn to_path(value: &String) -> PathBuf {
        value
            .replace(
                "$HOME",
                dirs::home_dir()
                    .expect("Cannot find home directory.")
                    .to_str()
                    .expect("Failed to convert path to string."),
            )
            .replace(
                "$CONFIG_DIR",
                dirs::config_dir()
                    .expect("Cannot find config directory.")
                    .to_str()
                    .expect("Failed to convert path to string."),
            )
            .into()
    }
}

#[derive(Deserialize, Debug, Clone)]
/// Dip configuration regarding to bundle feature.
pub struct BundleConfig {
    /// Path to your local bundle repository.
    repo: String,
    /// Section for the Version Manager
    pub vm: VMConfig,
}

impl BundleConfig {
    pub fn repo(&self) -> PathBuf {
        let p = Config::to_path(&self.repo);
        Config::ensure_dir(&p);

        p
    }

    pub fn config_file() -> PathBuf {
        let p = Config::config_dir().join("bundle");
        Config::ensure_dir(&p);

        p
    }

    pub fn bundle_dir(&self) -> PathBuf {
        let p = Config::config_dir().join("bundle");
        Config::ensure_dir(&p);

        p
    }

    pub fn installs_dir(&self) -> PathBuf {
        let p = self.bundle_dir().join("installs");
        Config::ensure_dir(&p);

        p
    }
}

#[derive(Deserialize, Debug, Clone)]
/// Version Manager related configurations
pub struct VMConfig {
<<<<<<< HEAD
    pub tailwindcss: VersionList,
    pub nodejs: VersionList,
=======
    /// All runtime versions
    pub runtime: VMRuntime,
}

#[derive(Deserialize, Debug, Clone)]
pub struct VMRuntime {
    /// [Tailwind CSS](https://tailwindcss.com/)
    pub tailwindcss: VersionSet,
    /// [Node.js](https://nodejs.org/)
    pub nodejs: VersionSet,
>>>>>>> 051d114 (Create installs directory when it does not exist)
}

pub type VersionSet = HashSet<String>;
