use crate::{plugin::BuildAction, resource::tool::Tool};
use dip::{
    bevy::{
        app::AppExit,
        ecs::{
            event::{EventReader, EventWriter},
            schedule::ParallelSystemDescriptorCoercion,
            system::Res,
        },
    },
    prelude::{AsyncActionPool, Plugin},
};
use std::{path::PathBuf, process::Command};

use super::AsyncAction;

pub struct HandlerPlugin;

impl Plugin for HandlerPlugin {
    fn build(&self, app: &mut dip::prelude::App) {
        app.add_event::<BuildApp>()
            .add_event::<CompileCss>()
            .add_system(handle_build)
            .add_system(compile_css.after(handle_build))
            .add_system(build_app.after(handle_build).after(compile_css));
    }
}

fn handle_build(
    mut actions: EventReader<BuildAction>,
    mut build_app: EventWriter<BuildApp>,
    mut compile_css: EventWriter<CompileCss>,
    async_action: Res<AsyncActionPool<AsyncAction>>,
) {
    for a in actions.iter() {
        let tailwind_config = PathBuf::from(&a.path).join("tailwind.config.js");

        if tailwind_config.is_file() {
            let tailwind = &Tool::Tailwind;

            if !tailwind.bin_path().is_file() {
                async_action.send(AsyncAction::install_and_build(tailwind));
            }

            compile_css.send(CompileCss {
                path: a.path.clone(),
                tool: tailwind.clone(),
            });
        }

        build_app.send(BuildApp {
            path: a.path.clone(),
        });
    }
}

fn compile_css(mut events: EventReader<CompileCss>) {
    for e in events.iter() {
        let mut cmd = Command::new(&e.tool.bin_path_str());

        cmd.current_dir(&e.path);
        cmd.args([
            "-i",
            "styles/globals.css",
            "-o",
            "public/globals.css",
            "-c",
            "tailwind.config.js",
        ]);

        let status = cmd.status().expect("failed to execute process");

        println!("CSS compiled");
        println!("{status}");
    }
}

fn build_app(mut events: EventReader<BuildApp>, mut app_exit: EventWriter<AppExit>) {
    for e in events.iter() {
        let mut cmd = Command::new("cargo");

        cmd.current_dir(&e.path).args(["build"]);

        let status = cmd.status().expect("failed to build app");

        println!("{status}");

        app_exit.send(AppExit);
    }
}

#[derive(Clone, Debug)]
struct BuildApp {
    pub path: String,
}

#[derive(Clone, Debug)]
struct CompileCss {
    pub path: String,
    pub tool: Tool,
}
