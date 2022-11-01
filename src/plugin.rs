mod action;
mod async_action;
mod cli;
mod handler;
mod tool;

use crate::plugin::{
    action::ActionPlugin,
    async_action::{AsyncAction, AsyncActionPlugin},
    cli::CliPlugin,
    handler::HandlerPlugin,
    tool::ToolPlugin,
};
use dip::bevy::app::{App, Plugin};

pub struct DipCliPlugin;

impl Plugin for DipCliPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CliPlugin::<AsyncAction>::application())
            .add_plugin(ActionPlugin)
            .add_plugin(AsyncActionPlugin)
            .add_plugin(HandlerPlugin)
            .add_plugin(ToolPlugin);
    }
}
