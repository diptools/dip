pub use crate::tool::{
    dotfiles::DotfilesPlugin, homebrew::HomebrewPlugin, script::ScriptPlugin,
    vm::VersionManagerPlugin,
};
use bevy::app::{App, Plugin};

pub struct UnixToolPlugin;

impl UnixToolPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for UnixToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ScriptPlugin);

        #[cfg(feature = "dotfiles")]
        app.add_plugin(DotfilesPlugin);

        #[cfg(feature = "brew")]
        app.add_plugin(HomebrewPlugin);

        #[cfg(feature = "vm")]
        app.add_plugin(VersionManagerPlugin::new());
    }
}
