pub use crate::tool::{
    dotfiles::DotfilesPlugin, homebrew::HomebrewPlugin, script::ScriptPlugin,
    tailwind::TailwindPlugin, vm::VersionManagerPlugin,
};
use bevy::app::{App, Plugin};
use std::{fmt::Debug, marker::PhantomData};

pub struct UnixToolPlugin<Config> {
    config: PhantomData<Config>,
}

impl<Config> UnixToolPlugin<Config> {
    pub fn new() -> Self {
        Self {
            config: PhantomData,
        }
    }
}

impl<Config> Plugin for UnixToolPlugin<Config>
where
    Config: 'static + Send + Sync + Debug,
{
    fn build(&self, app: &mut App) {
        app.add_plugin(ScriptPlugin);

        #[cfg(feature = "dotfiles")]
        app.add_plugin(DotfilesPlugin);

        #[cfg(feature = "brew")]
        app.add_plugin(HomebrewPlugin);

        #[cfg(feature = "tailwind")]
        app.add_plugin(TailwindPlugin);

        #[cfg(feature = "vm")]
        app.add_plugin(VersionManagerPlugin::<Config>::new());
    }
}
