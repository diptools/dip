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
    path = "examples/cli/config/config", // default: "config"
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
