mod tool;

use crate::tool::*;
use dip::{
    bevy::{
        app::App,
        ecs::event::EventReader,
        log::{self, LogPlugin},
    },
    cli::{CliPlugin, SubcommandPlugin},
};

fn main() {
    App::new()
        .add_plugin(CliPlugin)
        .add_plugin(ActionPlugin)
        .add_plugin(ToolActionPlugin)
        .add_plugin(ConfigActionPlugin)
        .add_plugin(LogPlugin)
        .add_system(handle_tool_install)
        .add_system(handle_config_add)
        .run();
}

#[derive(CliPlugin, clap::Parser)]
#[clap(version)]
struct Cli {
    #[clap(short, long)]
    path: Option<String>,

    #[clap(subcommand)]
    action: Action,
}

#[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
pub enum Action {
    #[clap(subcommand)]
    Tool(ToolAction),

    #[clap(subcommand)]
    Config(ConfigAction),
}

fn handle_config_add(mut events: EventReader<AddConfigAction>) {
    for e in events.iter() {
        log::info!("{e:#?}");
    }
}

#[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
pub enum ConfigAction {
    Add,
}
