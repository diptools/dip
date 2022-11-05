pub use crate::tool::{
    dotfiles::DotfilesPlugin, homebrew::HomebrewPlugin, tailwind::TailwindPlugin,
};
use bevy::app::{App, Plugin};

pub struct UnixToolPlugin;

impl Plugin for UnixToolPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "dotfiles")]
        app.add_plugin(DotfilesPlugin);

        #[cfg(feature = "brew")]
        app.add_plugin(HomebrewPlugin);

        #[cfg(feature = "tailwind")]
        app.add_plugin(TailwindPlugin);
    }
}
