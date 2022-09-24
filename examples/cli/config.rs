use dip::prelude::*;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
struct Settings {
    list: Vec<String>,
}

fn main() {
    App::new()
        .insert_resource(Config::new())
        .add_plugin(CliPlugin)
        .add_plugin(ActionPlugin)
        .add_system(handle_path)
        .add_system(handle_show)
        .run();
}

#[derive(CliPlugin, clap::Parser, Clone)]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(SubcommandPlugin, clap::Subcommand, Clone)]
pub enum Action {
    Path,
    Show,
}

fn handle_path(mut actions: EventReader<PathAction>, settings: Res<Settings>) {
    for _ in actions.iter() {
        println!("{settings:?}");
    }
}

fn handle_show(mut actions: EventReader<ShowAction>) {
    for action in actions.iter() {
        println!("{action:?}");
    }
}
