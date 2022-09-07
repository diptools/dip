use clap::Parser;
use dip::bevy::{
    app::App,
    ecs::event::EventReader,
    log::{self, LogPlugin},
};

fn main() {
    App::new()
        .add_plugin(CliPlugin)
        .add_plugin(LogPlugin)
        .add_system(handle_build)
        .add_system(handle_clean)
        .run();
}

// #[cli_plugin]
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct DipCli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Build,
    Clean,
}

fn handle_build(mut events: EventReader<Build>) {
    for _ in events.iter() {
        log::info!("build");
    }
}

fn handle_clean(mut events: EventReader<Clean>) {
    for _ in events.iter() {
        log::info!("clean");
    }
}

// generate with `#[cli_plugin]` proc macro
struct Build;
struct Clean;

struct CliPlugin;

impl dip::bevy::app::Plugin for CliPlugin {
    fn build(&self, app: &mut dip::bevy::app::App) {
        app.insert_resource(DipCli::parse())
            .add_event::<Build>()
            .add_event::<Clean>()
            .set_runner(|app| Self::runner(app));
    }
}

impl CliPlugin {
    fn runner(mut app: dip::bevy::app::App) {
        let cli = app.world.get_resource::<DipCli>().unwrap();

        match cli.command {
            Commands::Build => {
                app.world
                    .get_resource_mut::<dip::bevy::ecs::event::Events<Build>>()
                    .unwrap()
                    .send(Build);
            }
            Commands::Clean => {
                app.world
                    .get_resource_mut::<dip::bevy::ecs::event::Events<Clean>>()
                    .unwrap()
                    .send(Clean);
            }
        }

        app.update();
    }
}
