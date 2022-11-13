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
use std::{fmt::Debug, marker::PhantomData, path::PathBuf};

pub struct ToolPlugin<Config> {
    config: PhantomData<Config>,
}

impl<Config> ToolPlugin<Config> {
    pub fn new() -> Self {
        Self {
            config: PhantomData,
        }
    }
}

impl<Config> Plugin for ToolPlugin<Config>
where
    Config: 'static + Send + Sync + Debug,
{
    fn build(&self, app: &mut App) {
        app.add_event::<InstallTools>();

        #[cfg(target_family = "unix")]
        app.add_plugin(UnixToolPlugin::<Config>::new());
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
