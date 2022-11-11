use crate::tool::InstallTools;
use bevy::{
    app::{App, Plugin},
    ecs::event::{EventReader, EventWriter},
    log,
};

// Plugin
pub struct TailwindPlugin;

impl Plugin for TailwindPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TailwindInstalled>().add_system(install);
    }
}

fn install(mut events: EventReader<InstallTools>, mut installed: EventWriter<TailwindInstalled>) {
    for _e in events.iter() {
        log::warn!("TODO: Install Tool");

        installed.send(TailwindInstalled);
    }
}

// Events

pub struct TailwindInstalled;
