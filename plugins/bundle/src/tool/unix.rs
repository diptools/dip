use bevy::app::{App, Plugin};

#[cfg(feature = "brew")]
mod homebrew;

#[cfg(feature = "brew")]
use self::homebrew::HomebrewPlugin;

pub struct UnixToolPlugin;

impl UnixToolPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for UnixToolPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "brew")]
        app.add_plugin(HomebrewPlugin);
    }
}
