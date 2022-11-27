use crate::{ApplyBundle, BundleConfig, BundleStage, Bundler};
use bevy::{
    app::{App, Plugin},
    ecs::{event::EventReader, system::Res},
};
use cmd_lib::spawn_with_output;
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
    events.iter().for_each(|_e| {
        let script = Script::pre(config.clone());
        let action = "Pre script";

        if script.script_file_path().is_file() {
            println!("📌 {action}");

            if let Err(e) = script.run() {
                eprintln!("❌ Failed: {}: {e}", &script.schedule.to_string());
            } else {
                println!("✅ {action}");
            }
        } else {
            println!(
                "🟡 Skip: {action}: {} does not exists",
                &script.script_file_path().display()
            );
        }
    });
}

fn post_script(mut events: EventReader<ApplyBundle>, config: Res<BundleConfig>) {
    events.iter().for_each(|_e| {
        let script = Script::post(config.clone());
        let action = "Post script";

        if script.script_file_path().is_file() {
            println!("📌 {action}");

            if let Err(e) = script.run() {
                eprintln!("❌ Failed: {}: {e}", script.schedule.to_string());
            } else {
                println!("✅ {action}");
            }
        } else {
            println!(
                "🟡 Skip: {}: {action} does not exists",
                &script.script_file_path().display()
            );
        }
    });
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

    fn run(&self) -> anyhow::Result<()> {
        let file_path_string = &self.script_file_path().display().to_string();
        let mut script = spawn_with_output!(/bin/bash -C $file_path_string).unwrap();

        script.wait_with_pipe(&mut |pipe| {
            BufReader::new(pipe)
                .lines()
                .filter_map(|line| line.ok())
                .for_each(|f| println!("{f}"));
        })?;

        Ok(())
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

impl ToString for ScriptSchedule {
    fn to_string(&self) -> String {
        match self {
            ScriptSchedule::Pre => "pre".into(),
            ScriptSchedule::Post => "post".into(),
        }
    }
}
