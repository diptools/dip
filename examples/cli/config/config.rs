use config::{Config as ConfigRaw, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub env: String,
    pub backend: Backend,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        let base_path = "examples/cli/config/config";

        ConfigRaw::builder()
            .add_source(File::with_name(&format!("{base_path}/default")))
            .add_source(
                File::with_name(&format!("{base_path}/config/{}", run_mode)).required(false),
            )
            .add_source(File::with_name(&format!("{base_path}/local")))
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?
            .try_deserialize()
    }
}

#[derive(Debug, Deserialize)]
pub struct Backend {
    pub api_key: String,
    pub api_secret: String,
}
