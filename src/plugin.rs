mod async_action;
mod cli;
mod handler;
mod tool;

use crate::plugin::{async_action::*, cli::*, handler::*, tool::*};
use dip::bevy::app::{App, Plugin};

pub struct DipCliPlugin;

impl Plugin for DipCliPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CliPlugin::<AsyncAction>::application())
            .add_plugin(ActionPlugin)
            .add_plugin(AsyncActionPlugin)
            .add_plugin(ToolPlugin)
            .add_plugin(HandlerPlugin);
    }
}
