mod plugin;
mod resource;

use crate::{
    plugin::{ActionPlugin, AsyncAction, BuildAction, CliPlugin, ToolPlugin},
    resource::tool::Tool,
};
use dip::bevy::{
    app::{App, AppExit},
    ecs::event::{EventReader, EventWriter},
    log::LogPlugin,
};
use std::{path::PathBuf, process::Command};

fn main() {
    App::new()
        .add_plugin(CliPlugin::<AsyncAction>::continuous())
        .add_plugin(ActionPlugin)
        .add_plugin(ToolPlugin)
        .add_plugin(LogPlugin)
        .add_system(handle_build)
        .run();
}

fn handle_build(mut actions: EventReader<BuildAction>, mut app_exit: EventWriter<AppExit>) {
    for a in actions.iter() {
        if PathBuf::from(&a.path).join("tailwind.config.js").is_file() {
            let tool = Tool::Tailwind;
            let mut cmd = Command::new(&tool.bin_path_str());
            cmd.current_dir(&a.path);
            cmd.args([
                "-i",
                "styles/globals.css",
                "-o",
                "public/globals.css",
                "-c",
                "tailwind.config.js",
            ]);

            cmd.output().expect("failed to execute process");
            println!("Build completed");
        }

        app_exit.send(AppExit);
    }
}
