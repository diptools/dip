mod config;
pub mod tool;

use bevy::{
    app::{App, Plugin},
    ecs::{
        event::{EventReader, EventWriter},
        schedule::ParallelSystemDescriptorCoercion,
    },
};
pub use config::BundleConfigPlugin;
use tool::{InstallTools, ToolPlugin};

pub struct BundlePlugin;

impl Plugin for BundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ApplyBundle>()
            .add_event::<BundleApplied>()
            .add_plugin(BundleConfigPlugin)
            .add_plugin(ToolPlugin)
            .add_system(apply_bundle.after("apply_bundle"));
    }
}

// Events

pub struct ApplyBundle {
    pub verbose: bool,
}

pub struct BundleApplied;

fn apply_bundle(
    mut events: EventReader<ApplyBundle>,
    mut install_tools: EventWriter<InstallTools>,
) {
    events
        .iter()
        .map(|e| InstallTools { verbose: e.verbose })
        .for_each(|e| {
            install_tools.send(e);
        });
}
