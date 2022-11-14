mod action;

<<<<<<< HEAD
use crate::{
    cli::action::{
        ActionPlugin, ApplyBundleAction, BundleActionPlugin, CleanBundleAction, CliPlugin,
    },
    config::DipConfig,
=======
use crate::cli::action::{
    ActionPlugin, ApplyBundleAction, BundleActionPlugin, CleanBundleAction, CliPlugin,
>>>>>>> 0a64aae (Replace ConfigPlugin with BundleConfigPlugin)
};
use dip::{
    bevy::{
        app::{App, Plugin},
        ecs::event::{EventReader, EventWriter},
    },
    bundle::{ApplyBundle, BundlePlugin, CleanBundle},
    core::task::NoAsyncAction,
};
use std::path::PathBuf;

pub struct DipCliPlugin;

impl Plugin for DipCliPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CliPlugin::<NoAsyncAction>::oneshot())
            .add_plugin(ActionPlugin)
            .add_plugin(BundlePlugin)
            .add_plugin(BundleActionPlugin)
<<<<<<< HEAD
            .add_plugin(BundlePlugin::<DipConfig>::new())
=======
>>>>>>> 0a64aae (Replace ConfigPlugin with BundleConfigPlugin)
            .add_system(install_bundle)
            .add_system(clean_bundle);
    }
}

fn install_bundle(
    mut actions: EventReader<ApplyBundleAction>,
    mut apply: EventWriter<ApplyBundle>,
) {
    actions.iter().for_each(|a| {
        apply.send(ApplyBundle {
            path: PathBuf::from(&a.path),
        });
    });
}

fn clean_bundle(mut actions: EventReader<CleanBundleAction>, mut clean: EventWriter<CleanBundle>) {
    actions.iter().for_each(|a| {
        clean.send(CleanBundle {
            path: PathBuf::from(&a.path),
        })
    });
}
