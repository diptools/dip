use dip::{bevy::log::LogPlugin, prelude::*};

fn main() {
    App::new()
        .add_plugin(CliPlugin)
        .add_plugin(LogPlugin)
        .add_system(log_root_arg)
        .add_system(log_path_flag)
        .add_system(handle_hello)
        .add_system(handle_task)
        .add_system(handle_ping)
        .run();
}

#[derive(CliPlugin, clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    root_arg: Option<String>,

    #[clap(short, long)]
    path: Option<String>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, clap::Subcommand, Clone)]
enum Commands {
    Hello { name: Option<String> },
    Task(TaskArgs),
    Ping,
}

#[derive(clap::Args, Clone, Debug)]
struct TaskArgs {
    value: Option<String>,
}

fn log_root_arg(cli: Res<Cli>) {
    if let Some(arg) = &cli.root_arg {
        info!("root arg: {:?}", arg);
    }
}

fn log_path_flag(cli: Res<Cli>) {
    if let Some(path) = &cli.path {
        info!("path flag: {:?}", path);
    }
}

fn handle_hello(mut events: EventReader<Hello>) {
    for e in events.iter() {
        info!("Hello, {}!", e.name.clone().unwrap_or("world".to_string()));
    }
}

fn handle_task(mut events: EventReader<Task>) {
    for e in events.iter() {
        info!("Task: {e:?}");
    }
}

fn handle_ping(mut events: EventReader<Ping>) {
    for _ in events.iter() {
        info!("Pong !");
    }
}
