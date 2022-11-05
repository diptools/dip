mod dotfiles;
mod homebrew;
mod tailwind;

pub use self::{dotfiles::DotfilesPlugin, homebrew::HomebrewPlugin, tailwind::TailwindPlugin};
use bevy::{
    app::{App, Plugin},
    ecs::component::Component,
};
use std::path::PathBuf;

pub struct ToolPlugin;

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InstallTools>().add_event::<ApplyTools>();

        #[cfg(feature = "dotfiles")]
        app.add_plugin(DotfilesPlugin);

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
