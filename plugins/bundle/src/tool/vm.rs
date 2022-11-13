<<<<<<< HEAD
use crate::{ApplyBundle, BundleStage, CleanBundle};
use bevy::{
    app::{App, Plugin},
    ecs::event::EventReader,
=======
use std::{fmt::Debug, marker::PhantomData, path::PathBuf};

use crate::{ApplyBundle, BundleStage};
use bevy::{
    app::{App, Plugin},
    ecs::{event::EventReader, system::Res},
    log,
>>>>>>> 0f1f59e (Pass Config type to ConfigPlugin)
};

pub struct VersionManagerPlugin<Config> {
    config: PhantomData<Config>,
}

impl<Config> Plugin for VersionManagerPlugin<Config>
where
    Config: 'static + Send + Sync + Debug,
{
    fn build(&self, app: &mut App) {
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
    }
}

impl<Config> VersionManagerPlugin<Config> {
    pub fn new() -> Self {
        Self {
            config: PhantomData,
        }
    }
}

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
}
