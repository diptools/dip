<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
use crate::tool::InstallTools;
=======
use crate::{
    config::{BundleConfig, Config},
    schedule::BundleStage,
    ApplyBundle, CleanBundle,
};
>>>>>>> 051d114 (Create installs directory when it does not exist)
=======
use crate::{config::BundleConfig, schedule::BundleStage, ApplyBundle};
>>>>>>> e04d1b0 (Merge bundle config with cli arguments)
=======
use crate::{config::BundleConfig, schedule::BundleStage, ApplyBundle, Bundler};
>>>>>>> 51d7a93 (Parse path and url from config file)
use bevy::{
    app::{App, Plugin},
    ecs::event::{EventReader, EventWriter},
    log,
};
<<<<<<< HEAD
<<<<<<< HEAD
=======
use std::{fs, path::PathBuf};
>>>>>>> 051d114 (Create installs directory when it does not exist)
=======
use std::{collections::HashSet, fs, path::PathBuf};

use super::VersionManager;
>>>>>>> 51d7a93 (Parse path and url from config file)

// Plugin
pub struct TailwindPlugin;
impl Plugin for TailwindPlugin {
    fn build(&self, app: &mut App) {
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
        app.add_event::<TailwindInstalled>().add_system(install);
    }
}

fn install(mut events: EventReader<InstallTools>, mut installed: EventWriter<TailwindInstalled>) {
    for _e in events.iter() {
        log::warn!("TODO: Install Tool");

        installed.send(TailwindInstalled);
=======
        app.add_system_to_stage(BundleStage::Clean, clean)
=======
        app
            // .add_system_to_stage(BundleStage::Clean, clean)
>>>>>>> a47ed81 (Add ConfigStartupStage)
=======
        app.add_system_to_stage(BundleStage::Clean, clean)
>>>>>>> 51d7a93 (Parse path and url from config file)
            .add_system_to_stage(BundleStage::Apply, apply);
    }
}

fn clean(mut events: EventReader<ApplyBundle>, config: Res<BundleConfig>) {
    events.iter().for_each(|_e| {
        log::warn!("TODO: Implement clean system for Version Manager");
        let tw = TailwindCSS::from(config.clone());

        if let Ok(installs_dir) = tw.installs_dir().canonicalize() {
            let installs = fs::read_dir(installs_dir).expect("Failed to read installs/ directory");

            // iterate over each versions currently installed but removed from the user bundle config
            installs.map(Result::ok).for_each(|version| {
                println!("{version:?}");
            });
        } else {
            println!("Tailwind CSS is not installed");
        }
    });
}

fn apply(mut events: EventReader<ApplyBundle>, config: Res<BundleConfig>) {
    events.iter().for_each(|_e| {
        log::warn!("TODO: Implement install system for Version Manager");

        let tw = TailwindCSS::from(config.clone());
        log::info!("{:#?}", tw.bundle);
        tw.versions().for_each(|v| {
            log::info!("{v}");
        });
    });
}

struct TailwindCSS {
    bundle: PathBuf,
    versions: HashSet<String>,
}

impl Bundler for TailwindCSS {
    fn name() -> &'static str {
        "tailwindcss"
    }

<<<<<<< HEAD
    fn bundle_dir(&self) -> PathBuf {
        self.config.repo().join("bundle/vm")
    }

    fn installs_dir(&self) -> PathBuf {
        self.config.installs_dir().join("tailwindcss")
    }

    fn versions(&self) -> std::collections::hash_set::Iter<'_, std::string::String> {
        self.config.vm.runtime.tailwindcss.iter()
>>>>>>> 051d114 (Create installs directory when it does not exist)
=======
    fn bundle(&self) -> &PathBuf {
        &self.bundle
    }
}

impl VersionManager for TailwindCSS {
    fn versions(&self) -> std::collections::hash_set::Iter<'_, String> {
        self.versions.iter()
    }
}

impl From<BundleConfig> for TailwindCSS {
    fn from(config: BundleConfig) -> Self {
        Self {
            bundle: config.bundle_root().join("tailwindcss"),
            versions: config.vm.runtime.tailwindcss,
        }
>>>>>>> 51d7a93 (Parse path and url from config file)
    }
}

// Events

pub struct TailwindInstalled;
