mod homebrew;
mod tailwind;

use bevy::{
    app::{App, Plugin},
    ecs::component::Component,
};
pub use homebrew::{HomebrewApplied, HomebrewInstalled, HomebrewPlugin};
use std::path::PathBuf;
pub use tailwind::{TailwindInstalled, TailwindPlugin};

pub struct ToolPlugin;

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InstallTools>().add_event::<ApplyTools>();

        #[cfg(feature = "brew")]
        app.add_plugin(HomebrewPlugin);

        #[cfg(feature = "tailwind")]
        app.add_plugin(TailwindPlugin);
    }
}

// Events

pub struct InstallTools {
    pub verbose: bool,
    pub path: PathBuf,
}

pub struct ApplyTools {
    pub verbose: bool,
    pub path: PathBuf,
}

// Commponents

#[derive(Component)]
pub struct Tool;
