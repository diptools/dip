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

// #[derive(Parser, CliPlugin)]
// #[clap(author, version, about, long_about = None)]
#[dip::cli::plugin(author, version, about, long_about = None)]
struct DipCli {
    #[dip::cli::plugin(subcommand)]
    command: Commands,
}

// #[derive(clap::Subcommand, dip::cli::SubCommand)]
#[dip::cli::subcommand]
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
