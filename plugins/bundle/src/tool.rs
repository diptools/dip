#[cfg(feature = "dotfiles")]
mod dotfiles;
#[cfg(feature = "brew")]
mod homebrew;
#[cfg(feature = "scripts")]
mod script;
#[cfg(target_family = "unix")]
mod unix;
#[cfg(feature = "vm")]
mod vm;

pub use self::unix::UnixToolPlugin;
use bevy::{
    app::{App, Plugin},
    ecs::component::Component,
};

pub struct ToolPlugin;

impl ToolPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InstallTools>();

        #[cfg(target_family = "unix")]
        app.add_plugin(UnixToolPlugin::new());
    }
}

#[derive(Clone)]
pub struct InstallTools;

#[derive(Component)]
pub struct Tool;
