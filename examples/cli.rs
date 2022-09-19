use dip::{bevy::log::LogPlugin, prelude::*};

fn main() {
    App::new()
        .add_plugin(CliPlugin)
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

// generated

// // CliPlugin
// pub struct CliPlugin;

// impl ::bevy::app::Plugin for CliPlugin {
//     fn build(&self, app: &mut ::bevy::app::App) {
//         use clap::Parser;
//         // use ::dip::bevy::ecs::{
//         //     schedule::ParallelSystemDescriptorCoercion,
//         //     system::IntoSystem,
//         // };

//         let cli = Cli::parse();

//         app.add_plugin(::dip::core::schedule::UiSchedulePlugin)
//             .insert_resource(cli.action.clone())
//             .insert_resource(cli)
//             .add_event::<Action>()
//             .set_runner(|mut app| {
//                 app.update();
//             })
//             .add_system_to_stage(
//                 ::dip::core::schedule::UiStage::Action,
//                 convert_subcommand_to_event.before(handle_action),
//             );
//     }
// }

// fn convert_subcommand_to_event(
//     subcommand: ::dip::bevy::ecs::system::Res<Action>,
//     mut action: ::dip::bevy::ecs::event::EventWriter<Action>,
// ) {
//     action.send(subcommand.clone());
// }

// // Action Subcommand
// pub struct ActionPlugin;

// impl ::bevy::app::Plugin for ActionPlugin {
//     fn build(&self, app: &mut ::bevy::app::App) {
//         // use ::dip::bevy::ecs::{
//         //     schedule::ParallelSystemDescriptorCoercion,
//         //     system::IntoSystem,
//         // };

//         app.add_event::<PingAction>()
//             .add_event::<HelloAction>()
//             .add_event::<Hello2Action>()
//             .add_event::<TodoAction>()
//             .add_system_to_stage(
//                 ::dip::core::schedule::UiStage::Action,
//                 handle_action.before(handle_todo_action),
//             );
//     }
// }

// // Events
// #[derive(Clone, Debug)]
// pub struct PingAction;

// #[derive(Clone, Debug)]
// pub struct HelloAction {
//     name: Option<String>,
// }

// // only when type name is different (if variant_ident != first_field_ty)
// pub type Hello2Action = Hello2Args;

// pub fn handle_action(
//     mut events: ::dip::bevy::ecs::event::EventReader<Action>,
//     mut ping_action: ::dip::bevy::ecs::event::EventWriter<PingAction>,
//     mut hello_action: ::dip::bevy::ecs::event::EventWriter<HelloAction>,
//     mut hello2_action: ::dip::bevy::ecs::event::EventWriter<Hello2Action>,
//     mut todo_action: ::dip::bevy::ecs::event::EventWriter<TodoAction>,
// ) {
//     for e in events.iter() {
//         match e {
//             Action::Ping => {
//                 ping_action.send(PingAction);
//             }
//             Action::Hello { name } => hello_action.send(HelloAction { name: name.clone() }),
//             Action::Hello2(x) => {
//                 hello2_action.send(x.clone());
//             }
//             Action::Todo(x) => {
//                 todo_action.send(x.clone());
//             }
//         }
//     }
// }

// // TodoAction Subcommand
// pub struct TodoActionPlugin;

// impl ::bevy::app::Plugin for TodoActionPlugin {
//     fn build(&self, app: &mut ::bevy::app::App) {
//         // use ::dip::bevy::ecs::{
//         //     schedule::ParallelSystemDescriptorCoercion,
//         //     system::IntoSystem,
//         // };

//         app.add_event::<ListTodoAction>()
//             .add_event::<AddTodoAction>()
//             .add_event::<RemoveTodoAction>()
//             .add_system_to_stage(::dip::core::schedule::UiStage::Action, handle_todo_action);
//     }
// }

// // Events
// #[derive(Clone, Debug)]
// pub struct ListTodoAction;

// #[derive(Clone, Debug)]
// pub struct AddTodoAction {
//     name: Option<String>,
// }

// // pub type RemoveTodoAction = RemoveTodoAction;

// pub fn handle_todo_action(
//     mut events: ::dip::bevy::ecs::event::EventReader<TodoAction>,
//     mut list_todo_action: ::dip::bevy::ecs::event::EventWriter<ListTodoAction>,
//     mut add_todo_action: ::dip::bevy::ecs::event::EventWriter<AddTodoAction>,
//     mut remove_todo_action: ::dip::bevy::ecs::event::EventWriter<RemoveTodoAction>,
// ) {
//     for e in events.iter() {
//         match e {
//             TodoAction::List => {
//                 list_todo_action.send(ListTodoAction);
//             }
//             TodoAction::Add { name } => {
//                 add_todo_action.send(AddTodoAction { name: name.clone() });
//             }
//             TodoAction::Remove(x) => {
//                 remove_todo_action.send(x.clone());
//             }
//         }
//     }
// }
