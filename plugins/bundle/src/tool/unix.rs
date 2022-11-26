use bevy::app::{App, Plugin};

#[cfg(feature = "dotfiles")]
use crate::tool::dotfiles::DotfilesPlugin;
#[cfg(feature = "brew")]
use crate::tool::homebrew::HomebrewPlugin;
#[cfg(feature = "scripts")]
use crate::tool::script::ScriptPlugin;
#[cfg(feature = "vm")]
use crate::tool::vm::VersionManagerPlugin;

pub struct UnixToolPlugin;

impl UnixToolPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for UnixToolPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "scripts")]
        app.add_plugin(ScriptPlugin);

        #[cfg(feature = "dotfiles")]
        app.add_plugin(DotfilesPlugin);

        #[cfg(feature = "brew")]
        app.add_plugin(HomebrewPlugin);

        #[cfg(feature = "vm")]
        app.add_plugin(VersionManagerPlugin);
    }
}
