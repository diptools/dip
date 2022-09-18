mod tool;

use crate::tool::*;
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

#[derive(Subcommand, clap::Subcommand, Clone, Debug)]
pub enum Action {
    #[clap(subcommand)]
    Tool(ToolAction),
    #[clap(subcommand)]
    Config(ConfigAction),
}

fn handle_config_add(mut events: EventReader<ConfigActionAdd>) {
    for e in events.iter() {
        log::info!("{e:#?}");
    }
}

#[derive(Clone, Debug, Subcommand, clap::Subcommand)]
pub enum ConfigAction {
    Add,
}

// Generated

// CliPlugin
// pub struct CliPlugin;

// impl ::bevy::app::Plugin for CliPlugin {
//     fn build(&self, app: &mut ::bevy::app::App) {
//         use clap::Parser;
//         use dip::bevy::ecs::{schedule::ParallelSystemDescriptorCoercion, system::IntoSystem};

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

// // SubcommandPlugin
// pub struct ActionPlugin;

// impl ::bevy::app::Plugin for ActionPlugin {
//     fn build(&self, app: &mut ::bevy::app::App) {
//         use dip::bevy::ecs::{schedule::ParallelSystemDescriptorCoercion, system::IntoSystem};
//
//         app.add_event::<ToolAction>()
//             .add_event::<ConfigAction>()
//             .add_system_to_stage(
//                 ::dip::core::schedule::UiStage::Action,
//                 handle_action
//                     .before(handle_tool_action)
//                     .before(handle_config_action),
//             );
//     }
// }

// fn handle_action(
//     // mut events: ::dip::bevy::ecs::system::EventReader<Action>,
//     mut events: EventReader<Action>,
//     mut tool_action: ::dip::bevy::ecs::event::EventWriter<ToolAction>,
//     mut config_action: ::dip::bevy::ecs::event::EventWriter<ConfigAction>,
// ) {
//     for e in events.iter() {
//         match e.clone() {
//             Action::Tool(x) => {
//                 tool_action.send(x.clone());
//             }
//             Action::Config(x) => {
//                 config_action.send(x.clone());
//             }
//         }
//     }
// }

// #[derive(Clone, Debug)]
// pub struct ToolActionInstall;

// pub struct ToolActionPlugin;

// impl ::bevy::app::Plugin for ToolActionPlugin {
//     fn build(&self, app: &mut ::bevy::app::App) {
//         // use dip::bevy::ecs::schedule::ParallelSystemDescriptorCoercion;

//         app.add_event::<ToolActionInstall>()
//             .add_system_to_stage(::dip::core::schedule::UiStage::Action, handle_tool_action);
//     }
// }

// fn handle_tool_action(
//     mut events: ::dip::bevy::ecs::event::EventReader<ToolAction>,
//     mut tool_install: ::dip::bevy::ecs::event::EventWriter<ToolActionInstall>,
// ) {
//     for e in events.iter() {
//         match e {
//             ToolAction::Install => {
//                 tool_install.send(ToolActionInstall);
//             }
//         }
//     }
// }

// #[derive(Clone, Debug)]
// pub struct ConfigActionAdd;

// pub struct ConfigActionPlugin;

// impl ::bevy::app::Plugin for ConfigActionPlugin {
//     fn build(&self, app: &mut ::bevy::app::App) {
//         // use dip::bevy::ecs::schedule::ParallelSystemDescriptorCoercion;

//         app.add_event::<ConfigActionAdd>()
//             .add_system_to_stage(::dip::core::schedule::UiStage::Action, handle_config_action);
//     }
// }

// fn handle_config_action(
//     mut events: ::dip::bevy::ecs::event::EventReader<ConfigAction>,
//     mut config_add: ::dip::bevy::ecs::event::EventWriter<ConfigActionAdd>,
// ) {
//     for e in events.iter() {
//         match e {
//             ConfigAction::Add => {
//                 config_add.send(ConfigActionAdd);
//             }
//         }
//     }
// }
