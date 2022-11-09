mod cli;

use crate::cli::DipCliPlugin;
use dip::bevy::app::App;

fn main() {
    let mut app = App::new();

    app.add_plugin(DipCliPlugin).run();
}
