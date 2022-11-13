use crate::{ApplyBundle, BundleStage, CleanBundle};
use bevy::{
    app::{App, Plugin},
    ecs::event::EventReader,
};

pub struct VersionManagerPlugin;

impl Plugin for VersionManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(BundleStage::Apply, apply)
            .add_system_to_stage(BundleStage::Clean, clean);
    }
}

fn apply(mut events: EventReader<ApplyBundle>) {
    events.iter().for_each(|e| {
        todo!("Implement install system for Version Manager");
    });
}

fn clean(mut events: EventReader<CleanBundle>) {
    events.iter().for_each(|e| {
        todo!("Implement clean system for Version Manager");
    });
}
