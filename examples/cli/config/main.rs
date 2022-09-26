use dip::prelude::*;
use serde::Deserialize;

fn main() {
    App::new()
        .add_plugin(ConfigPlugin)
        .add_system(log_config)
        .run();
}

fn log_config(config: Res<Config>) {
    println!("{:#?}", *config);
}

#[derive(Debug, Deserialize)]
// #[config_plugin(
//     path = "examples/cli/config/config", // default: "config"
// )]
#[allow(dead_code)]
struct Config {
    base_url: String,
    backend: Backend,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Backend {
    api_key: String,
    api_secret: String,
}

pub struct ConfigPlugin;

impl ::bevy::app::Plugin for ConfigPlugin {
    fn build(&self, app: &mut ::bevy::app::App) {
        app.insert_resource(Config::new().unwrap());
    }
}

impl Config {
    pub fn new() -> Result<Self, ::config::ConfigError> {
        const PKG_NAME: &str = env!("CARGO_PKG_NAME");

        let home_dir = dirs::home_dir().unwrap();
        let home_dir_str = home_dir.to_str().unwrap();

        ::config::Config::builder()
            // default config file in binary
            .add_source(::config::File::from_str(
                include_str!("config/default.toml"),
                ::config::FileFormat::Toml,
            ))
            // $HOME/.config/{CARGO_PKG_NAME}
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
            .add_source(::config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}
