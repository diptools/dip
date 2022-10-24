use dip::{bevy::log::LogPlugin, prelude::*};
use todomvc::TodoMVCPlugin;

fn main() {
    let tailwind_css = std::fs::read_to_string("examples/todomvc/public/globals.css")
        .expect("Cannot find compiled Tailwind CSS file");

    App::new()
        .insert_non_send_resource(DesktopSettings::<NoRootProps> {
            custom_head: Some(format!("<style>{tailwind_css}</style>")),
            ..Default::default()
        })
        .add_plugin(TodoMVCPlugin)
        .add_plugin(LogPlugin)
        .run();
}
