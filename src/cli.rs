mod action;
mod async_action;

use crate::cli::{
    action::{ActionPlugin, ApplyBundleAction, BundleActionPlugin, CliPlugin},
    async_action::{AsyncAction, AsyncActionPlugin},
};
use dip::{
    bevy::{
        app::{App, Plugin},
        ecs::{
            event::{EventReader, EventWriter},
            schedule::ParallelSystemDescriptorCoercion,
        },
    },
    bundle::{ApplyBundle, BundlePlugin},
};

pub struct DipCliPlugin;

impl Plugin for DipCliPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CliPlugin::<AsyncAction>::application())
            .add_plugin(ActionPlugin)
            .add_plugin(BundleActionPlugin)
            .add_plugin(AsyncActionPlugin)
            .add_plugin(BundlePlugin)
            .add_system(apply_bundle.label("apply_bundle"));
    }
}

fn apply_bundle(mut actions: EventReader<ApplyBundleAction>, mut apply: EventWriter<ApplyBundle>) {
    actions.iter().for_each(|a| {
        println!("appyl_bundle");
        apply.send(ApplyBundle { verbose: a.verbose });
    })
}
