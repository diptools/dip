use anyhow::{Context, Result};
use bevy::{
    app::{App, Plugin},
    ecs::system::ResMut,
    log,
};
use config::{
    builder::{ConfigBuilder, DefaultState},
    File,
};
use dip_core::{config::ConfigPlugin as ConfigPluginRaw, prelude::ConfigStartupStage};
use serde::{de, Deserialize, Deserializer};
use std::{collections::HashSet, fs, path::PathBuf};
use url::Url;

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
    let config_file_path = BundleConfig::config_file_path();

    *builder = builder
        .clone()
        .add_source(File::with_name(&config_file_path.display().to_string()));
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
}

#[derive(Deserialize, Debug, Clone)]
/// Dip configuration regarding to bundle feature.
pub struct BundleConfig {
    /// URL of remote repository
    #[serde(default)]
    #[serde(deserialize_with = "ConfigParser::url_from_str")]
    pub repository: Option<Url>,

    /// Path to your local bundle repository.
    #[serde(deserialize_with = "ConfigParser::path_from_str")]
    bundle_root: PathBuf,

    /// Section for the Version Manager
    pub vm: VMConfig,
}

impl BundleConfig {
    pub fn config_file_path() -> PathBuf {
        let p = Config::config_dir().join("bundle");
        Config::ensure_dir(&p);

        p
    }

    pub fn bundle_root(&self) -> &PathBuf {
        Config::ensure_dir(&self.bundle_root);

        &self.bundle_root
    }

    pub fn installs_dir(&self) -> PathBuf {
        let p = self.bundle_root.join("installs");
        Config::ensure_dir(&p);

        p
    }

    pub fn set_bundle_root(&mut self, bundle_root: &String) -> anyhow::Result<()> {
        self.bundle_root = ConfigParser::to_path(&bundle_root.to_string())?;
        Ok(())
    }
}

struct ConfigParser;

impl ConfigParser {
    fn url_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Url>, D::Error> {
        let s = Deserialize::deserialize(d);
        match s {
            Ok(s) => match Url::parse(s) {
                Ok(url) => Ok(Some(url)),
                Err(e) => {
                    log::warn!("{e}");
                    Ok(None)
                }
            },
            Err(e) => {
                log::warn!("{e}");
                Ok(None)
            }
        }
    }

    fn path_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<PathBuf, D::Error> {
        let s: String = Deserialize::deserialize(d)?;

        match Self::to_path(&s) {
            Ok(path) => {
                if path.is_dir() {
                    Ok(path)
                } else {
                    Err(de::Error::custom("Bundle path is not a directory"))
                }
            }
            Err(_e) => Err(de::Error::custom("Failed to parse bundle directory path")),
        }
    }

    fn to_path(value: &String) -> Result<PathBuf> {
        let p = value
            .replace(
                "$HOME",
                dirs::home_dir()
                    .context("Cannot find home directory.")?
                    .to_str()
                    .context("Failed to convert path to string.")?,
            )
            .replace(
                "$CONFIG_DIR",
                dirs::config_dir()
                    .context("Cannot find config directory.")?
                    .to_str()
                    .context("Failed to convert path to string.")?,
            )
            .into();

        Ok(p)
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
