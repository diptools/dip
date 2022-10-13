mod action;
mod cli;
mod tool;

use crate::{action::ActionPlugin, cli::CliPlugin, tool::ToolPlugin};
use dip::{
    bevy::{app::App, log::LogPlugin},
    prelude::NoAsyncAction,
};

fn main() {
    App::new()
        .add_plugin(CliPlugin::<NoAsyncAction>::oneshot())
        .add_plugin(ActionPlugin)
        .add_plugin(ToolPlugin)
        .add_plugin(LogPlugin)
        .run();
}
