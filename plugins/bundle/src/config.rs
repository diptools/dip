use dip_core::config::{ConfigParser, ConfigUtil};
use reqwest::Url;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Clone)]
/// Dip configuration regarding to bundle feature.
pub struct BundleConfig {
    /// URL of remote repository
    #[serde(default)]
    #[serde(deserialize_with = "ConfigParser::url_from_str")]
    pub repository: Option<Url>,

    /// Path to local bundle repository.
    #[serde(deserialize_with = "ConfigParser::path_from_str")]
    root_dir: PathBuf,

    /// Section for the Version Manager.
    vm: VMConfig,

    /// Where all data resides. Runtime installs etc.
    #[serde(deserialize_with = "ConfigParser::path_from_str")]
    data_dir: PathBuf,
}

impl BundleConfig {
    pub fn root_dir(&self) -> PathBuf {
        ConfigUtil::ensure_dir(&self.root_dir);

        self.root_dir.clone()
    }

    pub fn install_root(&self) -> PathBuf {
        let p = self.data_dir.join("installs");
        ConfigUtil::ensure_dir(&p);

        p
    }

    pub fn shim_root(&self) -> PathBuf {
        let p = self.data_dir.join("shims");
        ConfigUtil::ensure_dir(&p);

        p
    }

    pub fn set_root_dir(&mut self, root_dir: &String) -> anyhow::Result<()> {
        self.root_dir = ConfigParser::to_path(&root_dir.to_string())?;
        Ok(())
    }

    pub fn runtime(&self) -> &VMRuntime {
        &self.vm.runtime
    }
}

#[derive(Deserialize, Debug, Clone)]
/// Version Manager related configurations
pub struct VMConfig {
    /// All runtime versions
    pub runtime: VMRuntime,
}

#[derive(Deserialize, Debug, Clone)]
pub struct VMRuntime {
    /// [Tailwind CSS](https://tailwindcss.com/)
    pub tailwindcss: VersionSet,
    /// [Node.js](https://nodejs.org/)
    pub nodejs: VersionSet,
}

pub type VersionSet = Vec<String>;
