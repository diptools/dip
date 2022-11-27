mod schedule;

use crate::schedule::ConfigSchedulePlugin;
use bevy::{
    app::{App, Plugin},
    ecs::system::{Commands, Res, ResMut},
};
use config::{builder::DefaultState, ConfigBuilder};
use serde::Deserialize;
use std::marker::PhantomData;

pub use crate::schedule::ConfigStartupStage;

#[derive(Debug)]
pub struct ConfigPlugin<Config> {
    config: PhantomData<Config>,
    default_paths: bool,
    env_prefix: Option<&'static str>,
    env_separator: &'static str,
    default_file_str: &'static str,
    default_file_format: ::config::FileFormat,
}

impl<Config> Plugin for ConfigPlugin<Config>
where
    Config: 'static + Send + Sync + Deserialize<'static>,
{
    fn build(&self, app: &mut App) {
        app.add_plugin(ConfigSchedulePlugin)
            .insert_resource(Self::builder(&self))
            .add_startup_system_to_stage(ConfigStartupStage::Build, build_config::<Config>);
    }
}

impl<Config> Default for ConfigPlugin<Config> {
    fn default() -> Self {
        Self {
            config: PhantomData,

            default_paths: true,
            env_prefix: None,
            env_separator: "__",
            default_file_str: "",
            default_file_format: ::config::FileFormat::Toml,
        }
    }
}

impl<Config> ConfigPlugin<Config> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_default_str(default_file_str: &'static str) -> Self {
        Self {
            default_file_str,
            ..Default::default()
        }
    }

    pub fn default_paths(mut self, default_paths: bool) -> Self {
        self.default_paths = default_paths;
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

    pub fn default_file_str(mut self, default_str: &'static str) -> Self {
        self.default_file_str = default_str;
        self
    }

    pub fn builder(&self) -> ConfigBuilder<DefaultState> {
        const PKG_NAME: &str = env!("CARGO_PKG_NAME");

        let home_dir = dirs::home_dir().unwrap();
        let home_dir_str = home_dir.to_str().unwrap();

        let mut builder = ::config::Config::builder();
        let mut env = ::config::Environment::default().separator(self.env_separator);

        if let Some(prefix) = &self.env_prefix {
            env = env.prefix(&prefix);
        }

        if self.default_paths {
            builder = builder
                // default config file in binary
                .add_source(::config::File::from_str(
                    self.default_file_str,
                    self.default_file_format,
                ))
                .add_source(
                    ::config::File::with_name(&format!(
                        "{home}/.config/{name}",
                        home = &home_dir_str,
                        name = PKG_NAME
                    ))
                    .required(false),
                )
                // $HOME/.config/{CARGO_PKG_NAME}/{CARGO_PKG_NAME}
                .add_source(
                    ::config::File::with_name(&format!(
                        "{home}/.config/{name}/{name}",
                        home = &home_dir_str,
                        name = PKG_NAME
                    ))
                    .required(false),
                )
                // $HOME/.{CARGO_PKG_NAME}
                .add_source(
                    ::config::File::with_name(&format!(
                        "{home}/.{name}",
                        home = &home_dir_str,
                        name = PKG_NAME
                    ))
                    .required(false),
                )
                // ./{CARGO_PKG_NAME}
                .add_source(
                    ::config::File::with_name(&format!("{name}", name = PKG_NAME)).required(false),
                )
                .add_source(env);
        }

        if let Ok(name) = std::env::var("CONFIG_PATH") {
            builder = builder.add_source(::config::File::with_name(&name));
        }

        builder
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
