<<<<<<< HEAD
<<<<<<< HEAD
use crate::{ApplyBundle, BundleStage, CleanBundle};
use bevy::{
    app::{App, Plugin},
    ecs::event::EventReader,
=======
use std::{fmt::Debug, marker::PhantomData, path::PathBuf};
=======
mod tailwind;
>>>>>>> 0a64aae (Replace ConfigPlugin with BundleConfigPlugin)

use std::path::PathBuf;

use crate::{
    config::{BundleConfig, VMConfig},
    tool::vm::tailwind::TailwindPlugin,
    ApplyBundle, BundleStage,
};
use bevy::{
    app::{App, Plugin},
    ecs::{event::EventReader, system::Res},
    log,
>>>>>>> 0f1f59e (Pass Config type to ConfigPlugin)
};

pub struct VersionManagerPlugin;

impl Plugin for VersionManagerPlugin {
    fn build(&self, app: &mut App) {
<<<<<<< HEAD
<<<<<<< HEAD
        app.add_system_to_stage(BundleStage::Apply, apply)
            .add_system_to_stage(BundleStage::Clean, clean);
    }
}

fn apply(mut events: EventReader<ApplyBundle>) {
    events.iter().for_each(|e| {
        todo!("Implement install system for Version Manager");
=======
        app.add_system_to_stage(BundleStage::Apply, apply::<Config>);
=======
        app.add_system_to_stage(BundleStage::Apply, apply);

        #[cfg(feature = "tailwind")]
        app.add_plugin(TailwindPlugin);
>>>>>>> 0a64aae (Replace ConfigPlugin with BundleConfigPlugin)
    }
}

impl VersionManagerPlugin {
    pub fn new() -> Self {
        Self
    }
}

<<<<<<< HEAD
fn apply<Config>(mut events: EventReader<ApplyBundle>, config: Res<Config>)
where
    Config: 'static + Send + Sync + Debug,
{
    events.iter().for_each(|_e| {
        log::warn!("Implement install system for Version Manager");
        log::info!("{:#?}", *config);
>>>>>>> 0f1f59e (Pass Config type to ConfigPlugin)
    });
}

fn clean(mut events: EventReader<CleanBundle>) {
    events.iter().for_each(|e| {
        todo!("Implement clean system for Version Manager");
    });
=======
fn apply(mut events: EventReader<ApplyBundle>, config: Res<BundleConfig>) {
    events.iter().for_each(|e| {
        log::warn!("TODO: Implement install system for Version Manager");

        let vm = VersionManager::new(e, config.vm.clone());
        log::info!("{:#?}", vm.config);
    });
}

struct VersionManager {
    project_path: PathBuf,
    config: VMConfig,
}

impl VersionManager {
    fn new(e: &ApplyBundle, config: VMConfig) -> Self {
        Self {
            project_path: e.project_path.clone(),
            config,
        }
    }

    fn bundle_path(&self) -> PathBuf {
        self.project_path.join("bundle/vm")
    }
>>>>>>> 0a64aae (Replace ConfigPlugin with BundleConfigPlugin)
}
