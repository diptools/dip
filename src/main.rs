mod action;
mod cli;
mod tool;

use crate::{action::ActionPlugin, cli::CliPlugin, tool::ToolPlugin};
use dip::bevy::{app::App, log::LogPlugin};

fn main() {
    App::new()
        .add_plugin(CliPlugin)
        .add_plugin(ActionPlugin)
        .add_plugin(ToolPlugin)
        .add_plugin(LogPlugin)
        .run();
}
