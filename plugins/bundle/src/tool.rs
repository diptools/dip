mod dotfiles;
mod homebrew;
mod script;
mod tailwind;
mod unix;
mod vm;

pub use self::unix::UnixToolPlugin;
use bevy::{
    app::{App, Plugin},
    ecs::component::Component,
};
use std::path::PathBuf;

pub struct ToolPlugin;

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InstallTools>();

        #[cfg(target_family = "unix")]
        app.add_plugin(UnixToolPlugin);
    }
}

// Events

#[derive(Clone)]
pub struct InstallTools {
    pub path: PathBuf,
}

// Commponents

#[derive(Component)]
pub struct Tool;
