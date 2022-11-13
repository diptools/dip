use bevy::{
    app::{App, Plugin},
    ecs::event::EventReader,
};
use cmd_lib::spawn_with_output;
use convert_case::{Case, Casing};
use std::{
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use crate::{ApplyBundle, BundleStage};

pub struct ScriptPlugin;

impl Plugin for ScriptPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(BundleStage::First, pre_script)
            .add_system_to_stage(BundleStage::Last, post_script);
    }
}

fn pre_script(mut events: EventReader<ApplyBundle>) {
    events.iter().for_each(|e| Script::pre(e.clone()).run());
}

fn post_script(mut events: EventReader<ApplyBundle>) {
    events.iter().for_each(|e| Script::post(e.clone()).run());
}

struct Script {
    event: ApplyBundle,
    schedule: ScriptSchedule,
}

impl Script {
    fn pre(event: ApplyBundle) -> Self {
        Self {
            event,
            schedule: ScriptSchedule::Pre,
        }
    }

    fn post(event: ApplyBundle) -> Self {
        Self {
            event,
            schedule: ScriptSchedule::Post,
        }
    }

    fn run(&self) {
        match self.find_file() {
            Ok(file_path) => {
                println!("📌 {} script", self.schedule.to_upper_camel());

                let file_path_str = file_path.display();
                let mut script = spawn_with_output!(/bin/bash -C $file_path_str).unwrap();

                let result = script.wait_with_pipe(&mut |pipe| {
                    BufReader::new(pipe)
                        .lines()
                        .filter_map(|line| line.ok())
                        .for_each(|f| println!("{f}"));
                });

                if let Err(e) = result {
                    println!("Failed to run {} script.", self.schedule.to_string());
                    eprintln!("{e}");
                } else {
                    println!("✅ {} script", self.schedule.to_upper_camel());
                }
            }
            Err(_e) => {
                self.skip();
            }
        }
    }

    fn skip(&self) {
        println!("🟡 Skip: {} script", &self.schedule.to_upper_camel());
        println!("{} does not exists.", &self.file_path().display());
    }

    fn find_file(&self) -> io::Result<PathBuf> {
        self.file_path().canonicalize()
    }

    fn file_path(&self) -> PathBuf {
<<<<<<< HEAD
        self.event
            .path
            .join(format!("bundle/scripts/{}", &self.file_name()))
=======
        self.bundle_path().join(&self.file_name())
>>>>>>> 0f1f59e (Pass Config type to ConfigPlugin)
    }

    fn file_name(&self) -> String {
        format!("{}.sh", self.schedule.to_string())
    }
}

enum ScriptSchedule {
    Pre,
    Post,
}

impl ScriptSchedule {
    fn to_upper_camel(&self) -> String {
        self.to_string().to_case(Case::UpperCamel)
    }
}

impl ToString for ScriptSchedule {
    fn to_string(&self) -> String {
        match self {
            ScriptSchedule::Pre => "pre".into(),
            ScriptSchedule::Post => "post".into(),
        }
    }
}
