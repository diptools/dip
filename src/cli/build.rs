use crate::cli::action::BuildAction;
use dip::{
    bevy::app::{App, Plugin},
    builder::BuildApp,
    prelude::{EventReader, EventWriter},
};
use dip_builder::BuilderPlugin;

/// Delegate CLI subcommands to BuilderPlugin.
pub struct BuildPlugin;

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BuilderPlugin).add_system(build);
    }
}

fn build(mut actions: EventReader<BuildAction>, mut build: EventWriter<BuildApp>) {
    actions.iter().for_each(|_a| {
        build.send(BuildApp);
    });
}
