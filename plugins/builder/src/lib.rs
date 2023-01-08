mod config;
mod schedule;

use anyhow::bail;
use bevy::{
    app::{App, Plugin},
    prelude::EventReader,
};
use serde::Deserialize;
use std::{path::PathBuf, str::FromStr};

pub use config::BuilderConfig;
pub use schedule::{BuilderSchedulePlugin, BuilderStage};

pub struct BuilderPlugin;

impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BuildApp>()
            .add_plugin(BuilderSchedulePlugin)
            .add_system_to_stage(BuilderStage::PreBuild, compile_css)
            .add_system_to_stage(BuilderStage::BuildApp, build_app);
    }
}

fn compile_css(mut events: EventReader<BuildApp>) {
    events.iter().for_each(|e| {
        if e.is_web() || e.is_desktop() {
            let tw = Tailwind::new();
        }
        // let tailwind_config = PathBuf::from(&e.path).join("tailwind.config.js");

        // if tailwind_config.is_file() {
        //     let tailwind = &Tool::Tailwind;

        //     if tailwind.bin_path().is_file() {
        //         compile_css.send(CompileCss {
        //             action: action.clone(),
        //             tool: tailwind.clone(),
        //         });
        //     } else {
        //         async_action.send(AsyncAction::install_and_build(tailwind, action.clone()));
        //     }
        // } else {
        //     build_app.send(BuildApp {
        //         action: action.clone(),
        //     });
        // }
        // todo!("build app system");
    });
}

struct Tailwind;

impl Tailwind {
    fn new() -> Self {
        Self
    }
}

fn build_app(mut events: EventReader<BuildApp>) {
    events.iter().for_each(|e| {
        if e.is_web() {
        } else if e.is_desktop() {
        }
    });
}

pub struct BuildApp {
    pub project_dir: PathBuf,
    pub platform: Platform,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Copy)]
pub enum Platform {
    Desktop,
    Web,
}

impl FromStr for Platform {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Platform, Self::Err> {
        match input {
            "desktop" => Ok(Platform::Desktop),
            "web" => Ok(Platform::Web),
            _ => bail!("Cannot find platform type: {}", input),
        }
    }
}

impl BuildApp {
    fn is_web(&self) -> bool {
        self.platform == Platform::Web
    }

    fn is_desktop(&self) -> bool {
        self.platform == Platform::Desktop
    }
}

// impl Plugin for BuildActionPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_event::<BuildApp>()
//             .add_event::<CompileCss>()
//             .add_system(handle_build)
//             .add_system(compile_css.after(handle_build))
//             .add_system(build_app.after(handle_build).after(compile_css));
//     }
// }

// fn handle_build(
//     mut actions: EventReader<BuildAction>,
//     mut build_app: EventWriter<BuildApp>,
//     mut compile_css: EventWriter<CompileCss>,
//     async_action: Res<AsyncActionPool<AsyncAction>>,
// ) {
//     for action in actions.iter() {
//         let tailwind_config = PathBuf::from(&action.path).join("tailwind.config.js");

//         if tailwind_config.is_file() {
//             let tailwind = &Tool::Tailwind;

//             if tailwind.bin_path().is_file() {
//                 compile_css.send(CompileCss {
//                     action: action.clone(),
//                     tool: tailwind.clone(),
//                 });
//             } else {
//                 async_action.send(AsyncAction::install_and_build(tailwind, action.clone()));
//             }
//         } else {
//             build_app.send(BuildApp {
//                 action: action.clone(),
//             });
//         }
//     }
// }

// fn build_app(mut events: EventReader<BuildApp>, mut app_exit: EventWriter<AppExit>) {
//     for BuildApp { action } in events.iter() {
//         let mut cmd = Command::new("cargo");

//         cmd.current_dir(fs::canonicalize(&action.path).unwrap())
//             .args(["build"]);

//         let output = cmd.output().expect("Could not execute cargo build");
//         log::trace!("{output:?}");

//         if output.status.success() {
//             println!("Build finished");
//         } else {
//             println!("Failed to build project");
//             println!("{}", String::from_utf8(output.stderr).unwrap());
//         }

//         app_exit.send(AppExit);
//     }
// }

// #[derive(Clone, Debug)]
// struct BuildApp {
//     pub action: BuildAction,
// }

// #[derive(Clone, Debug)]
// struct CompileCss {
//     pub action: BuildAction,
//     pub tool: Tool,
// }

// fn compile_css(
//     mut events: EventReader<CompileCss>,
//     mut build_app: EventWriter<BuildApp>,
//     mut app_exit: EventWriter<AppExit>,
// ) {
//     for CompileCss { action, tool } in events.iter() {
//         let mut cmd = Command::new(&tool.bin_path_str());

//         cmd.current_dir(fs::canonicalize(&action.path).unwrap());
//         cmd.args([
//             "-c",
//             &action.config,
//             "-i",
//             &action.input,
//             "-o",
//             &action.output,
//         ]);

//         if action.watch {
//             cmd.arg("-w");
//         }

//         let output = cmd
//             .output()
//             .expect("Could not execute compilation for Tailwind CSS");
//         log::trace!("{output:?}");

//         if output.status.success() {
//             println!("CSS compiled");

//             build_app.send(BuildApp {
//                 action: action.clone(),
//             })
//         } else {
//             println!("Failed to compile Tailwind CSS");
//             println!("{}", String::from_utf8(output.stderr).unwrap());

//             app_exit.send(AppExit);
//         }
//     }
// }
