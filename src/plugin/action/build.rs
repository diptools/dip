use crate::{
    plugin::{action::*, AsyncAction},
    resource::tool::Tool,
};
use dip::{
    bevy::{
        app::{App, AppExit, Plugin},
        ecs::prelude::*,
        log,
    },
    core::task::AsyncActionPool,
};
use std::{fs, path::PathBuf, process::Command};

pub struct BuildActionPlugin;

impl Plugin for BuildActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BuildApp>()
            .add_event::<CompileCss>()
            .add_system(handle_build)
            .add_system(compile_css.after(handle_build))
            .add_system(build_app.after(handle_build).after(compile_css));
    }
}

#[derive(clap::Args, Clone, Debug)]
pub struct BuildArgs {
    #[clap(short, long, default_value_t = String::from("."))]
    pub path: String,

    #[clap(short, long, default_value_t = String::from("tailwind.config.js"))]
    pub config: String,

    #[clap(short, long, default_value_t = String::from("styles/style.css"))]
    pub input: String,

    #[clap(short, long, default_value_t = String::from("static/style.css"))]
    pub output: String,

    #[clap(short, long)]
    pub watch: bool,
}

fn handle_build(
    mut actions: EventReader<BuildAction>,
    mut build_app: EventWriter<BuildApp>,
    mut compile_css: EventWriter<CompileCss>,
    async_action: Res<AsyncActionPool<AsyncAction>>,
) {
    for action in actions.iter() {
        let tailwind_config = PathBuf::from(&action.path).join("tailwind.config.js");

        if tailwind_config.is_file() {
            let tailwind = &Tool::Tailwind;

            if tailwind.bin_path().is_file() {
                compile_css.send(CompileCss {
                    action: action.clone(),
                    tool: tailwind.clone(),
                });
            } else {
                async_action.send(AsyncAction::install_and_build(tailwind, action.clone()));
            }
        } else {
            build_app.send(BuildApp {
                action: action.clone(),
            });
        }
    }
}

fn build_app(mut events: EventReader<BuildApp>, mut app_exit: EventWriter<AppExit>) {
    for BuildApp { action } in events.iter() {
        let mut cmd = Command::new("cargo");

        cmd.current_dir(fs::canonicalize(&action.path).unwrap())
            .args(["build"]);

        let output = cmd.output().expect("Could not execute cargo build");
        log::trace!("{output:?}");

        if output.status.success() {
            println!("Build finished");
        } else {
            println!("Failed to build project");
            println!("{}", String::from_utf8(output.stderr).unwrap());
        }

        app_exit.send(AppExit);
    }
}

#[derive(Clone, Debug)]
struct BuildApp {
    pub action: BuildAction,
}

#[derive(Clone, Debug)]
struct CompileCss {
    pub action: BuildAction,
    pub tool: Tool,
}

fn compile_css(
    mut events: EventReader<CompileCss>,
    mut build_app: EventWriter<BuildApp>,
    mut app_exit: EventWriter<AppExit>,
) {
    for CompileCss { action, tool } in events.iter() {
        let mut cmd = Command::new(&tool.bin_path_str());

        cmd.current_dir(fs::canonicalize(&action.path).unwrap());
        cmd.args([
            "-c",
            &action.config,
            "-i",
            &action.input,
            "-o",
            &action.output,
        ]);

        if action.watch {
            cmd.arg("-w");
        }

        let output = cmd
            .output()
            .expect("Could not execute compilation for Tailwind CSS");
        log::trace!("{output:?}");

        if output.status.success() {
            println!("CSS compiled");

            build_app.send(BuildApp {
                action: action.clone(),
            })
        } else {
            println!("Failed to compile Tailwind CSS");
            println!("{}", String::from_utf8(output.stderr).unwrap());

            app_exit.send(AppExit);
        }
    }
}
