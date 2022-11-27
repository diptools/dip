use crate::{ApplyBundle, BundleConfig, BundleStage, Bundler};
use bevy::{
    app::{App, Plugin},
    ecs::{event::EventReader, system::Res},
};
use cmd_lib::spawn_with_output;
use convert_case::{Case, Casing};
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub struct ScriptPlugin;

impl Plugin for ScriptPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(BundleStage::First, pre_script)
            .add_system_to_stage(BundleStage::Last, post_script);
    }
}

fn pre_script(mut events: EventReader<ApplyBundle>, config: Res<BundleConfig>) {
    events
        .iter()
        .for_each(|_e| Script::pre(config.clone()).run());
}

fn post_script(mut events: EventReader<ApplyBundle>, config: Res<BundleConfig>) {
    events
        .iter()
        .for_each(|_e| Script::post(config.clone()).run());
}

struct Script {
    bundle_config: BundleConfig,
    schedule: ScriptSchedule,
}

impl Script {
    fn pre(bundle_config: BundleConfig) -> Self {
        Self {
            bundle_config,
            schedule: ScriptSchedule::Pre,
        }
    }

    fn post(bundle_config: BundleConfig) -> Self {
        Self {
            bundle_config,
            schedule: ScriptSchedule::Post,
        }
    }

    fn run(&self) {
        let script_file_path = self.script_file_path();
        if script_file_path.is_file() {
            println!("📌 {} script", self.schedule.to_upper_camel());

            let file_path_str = script_file_path.display();
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
        } else {
            self.skip();
        }
    }

    fn skip(&self) {
        println!(
            "🟡 Skip: {} script: {} does not exists.",
            &self.schedule.to_upper_camel(),
            &self.script_file_path().display()
        );
    }

    fn script_file_path(&self) -> PathBuf {
        self.bundle_dir().join(&self.script_name())
    }

    fn script_name(&self) -> String {
        format!("{}.sh", self.schedule.to_string())
    }
}

impl Bundler for Script {
    fn key() -> &'static str {
        "scripts"
    }

    fn name() -> &'static str {
        "Scripts"
    }

    fn bundle_config(&self) -> &BundleConfig {
        &self.bundle_config
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
