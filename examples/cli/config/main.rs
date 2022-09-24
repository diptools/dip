mod config;

use crate::config::Config;
use dip::prelude::*;

fn main() {
    App::new()
        .insert_resource(Config::new().unwrap())
        .add_plugin(CliPlugin)
        .add_plugin(ActionPlugin)
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
