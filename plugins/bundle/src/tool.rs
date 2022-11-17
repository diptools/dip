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

// Events

#[derive(Clone)]
<<<<<<< HEAD
pub struct InstallTools {
    pub path: PathBuf,
}

=======
pub struct InstallTools;
>>>>>>> e04d1b0 (Merge bundle config with cli arguments)
// Commponents

#[derive(Component)]
pub struct Tool;
