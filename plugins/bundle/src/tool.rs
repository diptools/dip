mod homebrew;
mod tailwind;

use crate::BundleApplied;
use bevy::{
    app::{App, Plugin},
    ecs::{component::Component, event::EventWriter, query::With, system::Query},
};
pub use homebrew::{HomebrewApplied, HomebrewInstalled, HomebrewPlugin};
pub use tailwind::{TailwindInstalled, TailwindPlugin};

pub struct ToolPlugin;

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InstallTools>()
            .add_event::<ToolsInstalled>()
            .add_system(check_tools_installed)
            .add_system(check_bundle_applied);

        #[cfg(feature = "brew")]
        app.add_plugin(HomebrewPlugin);

        #[cfg(feature = "tailwind")]
        app.add_plugin(TailwindPlugin);
    }
}

// Events

pub struct InstallTools {
    pub verbose: bool,
}

pub struct ToolsInstalled;

// Commponents

#[derive(Component)]
pub struct Tool;

#[derive(Component)]
pub struct ToolInstalled;

#[derive(Component)]
pub struct ToolApplied;

// Systems

fn check_tools_installed(
    query: Query<Option<&ToolInstalled>, With<Tool>>,
    mut installed: EventWriter<ToolsInstalled>,
) {
    let not_installed = query.iter().find(|installed| installed.is_none());

    if not_installed.is_none() {
        installed.send(ToolsInstalled);
    }
}

fn check_bundle_applied(
    query: Query<Option<&ToolApplied>, With<Tool>>,
    mut applied: EventWriter<BundleApplied>,
) {
    let not_applied = query.iter().find(|applied| applied.is_none());

    if not_applied.is_none() {
        applied.send(BundleApplied);
    }
}
