mod cli;

use crate::cli::DipCliPlugin;
use dip::bevy::{
    app::App,
    log::{Level, LogPlugin},
};

fn main() {
    let mut app = App::new();

    #[cfg(debug_assertions)]
    let log_plugin = LogPlugin {
        filter: "info,dip=debug".into(),
        level: Level::DEBUG,
        ..LogPlugin::default()
    };

    #[cfg(not(debug_assertions))]
    let log_plugin = LogPlugin {
        filter: "warn".into(),
        level: Level::WARN,
        ..LogPlugin::default()
    };

    app.add_plugins((DipCliPlugin, log_plugin)).run();
}
