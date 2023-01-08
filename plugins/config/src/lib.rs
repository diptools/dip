mod parser;
mod schedule;
mod util;

use bevy::{
    app::{App, Plugin},
    ecs::system::{Commands, Res, ResMut},
};
use config::{Config as ConfigRaw, Environment, File, FileFormat};
use serde::Deserialize;
use std::path::PathBuf;

pub use parser::ConfigParser;
pub use schedule::{ConfigSchedulePlugin, ConfigStartupStage};
pub use util::ConfigUtil;

pub trait Configurable {}

pub struct ConfigPlugin {
    env_prefix: Option<&'static str>,
    env_separator: &'static str,

    sources: Vec<Box<dyn Configurable + 'static + Send + Sync>>,
}

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ConfigSchedulePlugin);

        for s in &self.sources {
            app.insert_resource(s);
        }
    }
}

impl Default for ConfigPlugin {
    fn default() -> Self {
        Self {
            env_prefix: None,
            env_separator: "__",

            sources: vec![],
        }
    }
}

impl ConfigPlugin {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<Config: 'static + Send + Sync + Deserialize<'static> + Configurable>(
        mut self,
        file_path: &PathBuf,
        default_str: &'static str,
    ) -> Self {
        let builder = ConfigRaw::builder()
            .add_source(File::from_str(default_str, FileFormat::Toml))
            .add_source(File::with_name(&file_path.display().to_string()).required(false));

        let mut env = Environment::default().separator(self.env_separator);

        if let Some(prefix) = &self.env_prefix {
            env = env.prefix(&prefix);
        }

        let config = builder
            .add_source(env)
            .build()
            .unwrap()
            .try_deserialize::<Config>()
            .unwrap();

        self.sources.push(Box::new(config));

        self
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

pub fn build_config<Config>(
    builder: Res<config::builder::ConfigBuilder<config::builder::DefaultState>>,
    config: Option<ResMut<Config>>,
    mut commands: Commands,
) where
    Config: 'static + Send + Sync + Deserialize<'static>,
{
    let c = builder
        .clone()
        .build()
        .unwrap()
        .try_deserialize::<'static, Config>()
        .expect("Failed to parse config");

    if config.is_none() {
        commands.insert_resource(c);
    }
}
