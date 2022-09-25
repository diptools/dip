use dip::prelude::*;
use serde::Deserialize;

fn main() {
    App::new()
        .add_plugin(CliPlugin)
        .add_plugin(ActionPlugin)
        .add_plugin(ConfigPlugin)
        .add_system(handle_show_config)
        .run();
}

#[derive(CliPlugin, clap::Parser, Clone)]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(SubcommandPlugin, clap::Subcommand, Clone)]
pub enum Action {
    Show,
}

fn handle_show_config(mut actions: EventReader<ShowAction>, config: Res<Config>) {
    for _ in actions.iter() {
        println!("{:#?}", *config);
    }
}

#[derive(Debug, Deserialize)]
#[config_plugin(
    path = "examples/cli/config/config", // default: "config/"
)]
#[allow(dead_code)]
struct Config {
    backend: Backend,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Backend {
    api_key: String,
    api_secret: String,
}

// generated

// pub struct ConfigPlugin;

// impl ::bevy::app::Plugin for ConfigPlugin {
//     fn build(&self, app: &mut ::bevy::app::App) {
//         app.insert_resource(Config::new().unwrap());
//     }
// }

// impl Config {
//     pub fn new() -> Result<Self, ::config::ConfigError> {
//         let run_mode = ::std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
//         let base_path = "examples/cli/config/config";

//         ::config::Config::builder()
//             .add_source(::config::File::with_name(&format!("{}/default", base_path)))
//             .add_source(
//                 ::config::File::with_name(&format!("{}/config/{}", base_path, run_mode))
//                     .required(false),
//             )
//             .add_source(::config::File::with_name(&format!("{}/local", base_path)))
//             .add_source(::config::Environment::with_prefix("APP").separator("__"))
//             .build()?
//             .try_deserialize()
//     }
// }
