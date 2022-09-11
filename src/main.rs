use dip::{
    bevy::{
        app::App,
        ecs::event::EventReader,
        log::{self, LogPlugin},
    },
    cli::{CliPlugin, Subcommand},
};

fn main() {
    App::new()
        .add_plugin(CliPlugin)
        .add_plugin(LogPlugin)
        .add_system(handle_build)
        .add_system(handle_clean)
        .add_system(handle_hello)
        .run();
}

#[derive(CliPlugin, clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct DipCli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Subcommand, Clone)]
enum Commands {
    Hello { name: Option<String> },
    Build(BuildArgs),
    Clean,
}

#[derive(clap::Args, Clone, Debug)]
struct BuildArgs {
    value: Option<String>,
}

fn handle_build(mut events: EventReader<Build>) {
    for e in events.iter() {
        log::info!("build: {e:?}");
    }
}

fn handle_clean(mut events: EventReader<Clean>) {
    for _ in events.iter() {
        log::info!("clean");
    }
}

fn handle_hello(mut events: EventReader<Hello>) {
    for e in events.iter() {
        log::info!("Hello, {}!", e.name.clone().unwrap_or("world".to_string()));
    }
}
