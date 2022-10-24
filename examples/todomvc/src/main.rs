use dip::{bevy::log::LogPlugin, prelude::*};
use todomvc::TodoMVCPlugin;

fn main() {
    App::new()
        .insert_non_send_resource(DesktopSettings::<NoRootProps> {
            custom_head: Some(format!(
                "<style>{}</style>",
                include_str!("../public/main.css")
            )),
            ..Default::default()
        })
        .add_plugin(TodoMVCPlugin)
        .add_plugin(LogPlugin)
        .run();
}
