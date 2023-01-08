use crate::cli::action::{ApplyBundleAction, BundleActionPlugin};
use dip::{
    bevy::{
        app::{App, Plugin},
        ecs::{
            event::{EventReader, EventWriter},
            system::ResMut,
        },
    },
    bundle::{ApplyBundle, BundleConfig, BundlePlugin as BundlePluginRaw},
};

/// Delegate CLI subcommands to BundlePlugin.
pub struct BundlePlugin;

impl Plugin for BundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BundleActionPlugin)
            .add_plugin(BundlePluginRaw)
            .add_system(install_bundle);
    }
}

fn install_bundle(
    mut actions: EventReader<ApplyBundleAction>,
    mut apply: EventWriter<ApplyBundle>,
    mut config: ResMut<BundleConfig>,
) {
    actions.iter().for_each(|a| {
        if let Some(value) = &a.bundle_root {
            config
                .set_root_dir(&value)
                .expect("Bundle root is not a directry");
        }

        apply.send(ApplyBundle);
    });
}
