use dip::{bevy::log::LogPlugin, prelude::*};

fn main() {
    App::new()
        .add_plugin(CliPlugin::<NoAsyncAction>::oneshot())
        .add_plugin(ActionPlugin)
        .add_plugin(TodoActionPlugin)
        .add_plugin(LogPlugin)
        .add_system(log_root_arg)
        .add_system(log_path_flag)
        .add_system(handle_hello)
        .add_system(handle_hello2)
        .add_system(handle_ping)
        .add_system(handle_add_todo)
        .add_system(handle_remove_todo)
        .add_system(handle_list_todo)
        .run();
}

#[derive(CliPlugin, clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    root_arg: Option<String>,

    #[clap(short, long)]
    path: Option<String>,

    #[clap(subcommand)]
    action: Action,
}

#[derive(SubcommandPlugin, clap::Subcommand, Clone)]
pub enum Action {
    // Unit
    Ping,
    // Named
    Hello {
        name: Option<String>,
    },
    // Unnamed
    Hello2(Hello2Args),

    // Subsubcommand
    #[clap(subcommand)]
    Todo(TodoAction),
}

#[derive(clap::Args, Clone, Debug)]
pub struct Hello2Args {
    name: Option<String>,
}

#[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
pub enum TodoAction {
    // Unit
    List,
    // Named
    Add { name: Option<String> },
    // Unnamed
    Remove(RemoveTodoAction),
}

#[derive(clap::Args, Clone, Debug)]
pub struct RemoveTodoAction {
    name: Option<String>,
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

fn handle_hello(mut events: EventReader<HelloAction>) {
    for e in events.iter() {
        info!("Hello, {}!", e.name.clone().unwrap_or("world".to_string()));
    }
}

fn handle_hello2(mut events: EventReader<Hello2Action>) {
    for e in events.iter() {
        info!("Hello, {}!", e.name.clone().unwrap_or("world".to_string()));
    }
}

fn handle_ping(mut events: EventReader<PingAction>) {
    for _ in events.iter() {
        info!("Pong !");
    }
}

fn handle_add_todo(mut events: EventReader<AddTodoAction>) {
    for e in events.iter() {
        info!("{e:?}");
        info!(
            "AddTodoAction: {}",
            e.name.clone().unwrap_or("<no-name>".to_string())
        );
    }
}

fn handle_remove_todo(mut events: EventReader<RemoveTodoAction>) {
    for e in events.iter() {
        info!("{e:?}");
    }
}

fn handle_list_todo(mut events: EventReader<ListTodoAction>) {
    for e in events.iter() {
        info!("{e:?}");
    }
}
