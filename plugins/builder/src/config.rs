use crate::Platform;
use dip_core::config::ConfigParser;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Clone)]
pub struct BuilderConfig {
    #[serde(deserialize_with = "ConfigParser::path_from_str")]
    pub project_dir: PathBuf,

    /// Platform to target. Default to desktop.
    pub platform: Platform,

    /// Output directory. Default to `dist/`
    pub out_dir: PathBuf,

    /// Asset resource directory. Default to `public/`
    pub asset_dir: PathBuf,
}
