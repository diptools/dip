mod action;

use crate::cli::action::{
    ActionPlugin, ApplyBundleAction, BundleActionPlugin, CleanBundleAction, CliPlugin,
};
use dip::{
    bevy::{
        app::{App, Plugin},
        ecs::event::{EventReader, EventWriter},
    },
    bundle::{ApplyBundle, BundlePlugin},
    core::task::NoAsyncAction,
};
use dip_bundle::CleanBundle;
use std::path::PathBuf;

pub struct DipCliPlugin;

impl Plugin for DipCliPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CliPlugin::<NoAsyncAction>::oneshot())
            .add_plugin(ActionPlugin)
            .add_plugin(BundleActionPlugin)
            .add_plugin(BundlePlugin)
            .add_system(apply_bundle)
            .add_system(clean_bundle);
    }
}

fn apply_bundle(mut actions: EventReader<ApplyBundleAction>, mut apply: EventWriter<ApplyBundle>) {
    actions.iter().for_each(|a| {
        apply.send(ApplyBundle {
            // verbose: a.verbose,
            verbose: true,
            path: PathBuf::from(&a.path),
        });
    });
}

fn clean_bundle(mut actions: EventReader<CleanBundleAction>, mut clean: EventWriter<CleanBundle>) {
    actions.iter().for_each(|a| {
        clean.send(CleanBundle {
            // verbose: a.verbose,
            verbose: true,
            path: PathBuf::from(&a.path),
        })
    });
}
