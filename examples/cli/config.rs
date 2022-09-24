use dip::prelude::*;

fn main() {
    App::new()
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

fn handle_path(mut actions: EventReader<PathAction>) {
    for action in actions.iter() {
        println!("{action:?}");
    }
}

fn handle_show(mut actions: EventReader<ShowAction>) {
    for action in actions.iter() {
        println!("{action:?}");
    }
}
