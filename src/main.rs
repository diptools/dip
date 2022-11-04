mod cli;
mod resource;

use crate::cli::DipCliPlugin;
use dip::bevy::{
    app::App,
    log::{LogPlugin, LogSettings},
};

fn main() {
    let mut app = App::new();

    #[cfg(debug_assertions)]
    app.insert_resource(LogSettings {
        filter: "info,dip=debug".into(),
        level: bevy::log::Level::DEBUG,
    });

    #[cfg(not(debug_assertions))]
    app.insert_resource(LogSettings {
        filter: "warn".into(),
        level: bevy::log::Level::WARN,
    });

    app.add_plugin(DipCliPlugin).add_plugin(LogPlugin).run();
}
