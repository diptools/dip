use dip::{
    bevy::{
        app::App,
        ecs::{event::EventReader, system::Res},
        log::{self, LogPlugin},
    },
    cli::{CliPlugin, Subcommand},
};

mod tool;

fn main() {
    App::new()
        .add_plugin(CliPlugin)
        .add_plugin(LogPlugin)
        .add_system(handle_tool)
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

#[derive(Subcommand, clap::Subcommand, Clone)]
enum Action {
    #[clap(subcommand)]
    Tool(tool::ToolInstall),
}

fn handle_tool(mut events: EventReader<tool::ToolInstall>) {
    for e in events.iter() {
        log::info!("Tool, {:#?}!", e);
    }
}
