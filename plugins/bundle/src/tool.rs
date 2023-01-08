#[cfg(feature = "dotfiles")]
mod dotfiles;
#[cfg(feature = "scripts")]
mod script;
#[cfg(target_family = "unix")]
mod unix;
#[cfg(feature = "vm")]
mod vm;

use bevy::app::{App, Plugin};
#[cfg(feature = "dotfiles")]
use dotfiles::DotfilesPlugin;
#[cfg(feature = "scripts")]
use script::ScriptPlugin;
#[cfg(feature = "vm")]
use vm::VersionManagerPlugin;

#[cfg(target_family = "unix")]
pub use unix::UnixToolPlugin;

pub struct UniversalToolPlugin;

impl Plugin for UniversalToolPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "dotfiles")]
        app.add_plugin(DotfilesPlugin);

        #[cfg(feature = "scripts")]
        app.add_plugin(ScriptPlugin);

        #[cfg(feature = "vm")]
        app.add_plugin(VersionManagerPlugin);
    }
}
