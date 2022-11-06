mod dotfiles;
mod homebrew;
mod script;
mod tailwind;
mod unix;

pub use self::unix::UnixToolPlugin;
use bevy::{
    app::{App, Plugin},
    ecs::component::Component,
};
use std::path::PathBuf;

pub struct ToolPlugin;

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InstallTools>().add_event::<ApplyTools>();

        #[cfg(target_family = "unix")]
        app.add_plugin(UnixToolPlugin);
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
