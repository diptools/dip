use crate::cli::action::BuildAction;
use dip::{
    bevy::{
        app::{App, Plugin},
        ecs::system::Res,
    },
    builder::BuildApp,
    core::config::ConfigParser,
    prelude::{EventReader, EventWriter},
};
use dip_builder::{BuilderConfig, BuilderPlugin};

/// Delegate CLI subcommands to BuilderPlugin.
pub struct BuildPlugin;

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BuilderPlugin).add_system(build);
    }
}

/// Delegate BuildAction as BuildApp event.
fn build(
    mut actions: EventReader<BuildAction>,
    mut build: EventWriter<BuildApp>,
    config: Res<BuilderConfig>,
) {
    actions.iter().for_each(|action| {
        let project_dir = if let Some(path_str) = action.project_dir.clone() {
            ConfigParser::to_path(&path_str).expect("Failed to parse project_dir.")
        } else {
            config.project_dir.clone()
        };

        build.send(BuildApp {
            project_dir,
            platform: action.platform.unwrap_or_else(|| config.platform),
        });
    });
}
