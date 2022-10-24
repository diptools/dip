mod plugin;
mod resource;

use crate::plugin::DipCliPlugin;
use dip::bevy::{app::App, log::LogPlugin};

fn main() {
    App::new()
        .add_plugin(DipCliPlugin)
        .add_plugin(LogPlugin)
        .run();
}
