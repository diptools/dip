mod plugin;
mod resource;

use crate::plugin::{ActionPlugin, AsyncAction, CliPlugin, ToolPlugin};
use dip::bevy::{app::App, log::LogPlugin};

fn main() {
    App::new()
        .add_plugin(CliPlugin::<AsyncAction>::continuous())
        .add_plugin(ActionPlugin)
        .add_plugin(ToolPlugin)
        .add_plugin(LogPlugin)
        .run();
}
