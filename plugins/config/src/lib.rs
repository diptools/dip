mod parser;
mod schedule;
mod util;

use config::{Config as ConfigRaw, Environment, File, FileFormat};
use serde::Deserialize;
use std::path::PathBuf;

pub use parser::ConfigParser;
pub use schedule::{ConfigSchedulePlugin, ConfigStartupStage};
pub use util::ConfigUtil;

pub struct SimpleConfig {
    env_prefix: Option<&'static str>,
    env_separator: &'static str,
}

impl Default for SimpleConfig {
    fn default() -> Self {
        Self {
            env_prefix: None,
            env_separator: "__",
        }
    }
}

impl SimpleConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build<Config: Deserialize<'static>>(
        &self,
        file_path: &PathBuf,
        default_str: &'static str,
    ) -> Config {
        let builder = ConfigRaw::builder()
            .add_source(File::from_str(default_str, FileFormat::Toml))
            .add_source(File::with_name(&file_path.display().to_string()).required(false));

        let mut env = Environment::default().separator(self.env_separator);

        if let Some(prefix) = &self.env_prefix {
            env = env.prefix(&prefix);
        }

        builder
            .add_source(env)
            .build()
            .unwrap()
            .try_deserialize::<Config>()
            .unwrap()
    }

    pub fn env_prefix(mut self, prefix: &'static str) -> Self {
        self.env_prefix = Some(prefix);
        self
    }

    pub fn env_separator(mut self, separator: &'static str) -> Self {
        self.env_separator = separator;
        self
    }
}
