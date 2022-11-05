use crate::{ApplyBundle, BundleStage};
use bevy::{
    app::{App, Plugin},
    ecs::event::EventReader,
    log,
};

// Plugin

pub struct DotfilesPlugin;

impl Plugin for DotfilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(BundleStage::Apply, apply);
    }
}

fn apply(mut events: EventReader<ApplyBundle>) {
    events.iter().for_each(|_e| {
        log::info!("TODO: apply dotfiles");
    });
}
