use bevy::{log::LogPlugin, prelude::*};
use bevy_dioxus::desktop::prelude::*;
use dioxus::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Props Example".to_string(),
            ..Default::default()
        })
        .add_plugin(DioxusPlugin::<(), (), RootProps>::new(app))
        .add_plugin(LogPlugin)
        .run();
}

#[derive(Props, PartialEq, Clone)]
struct RootProps {
    name: String,
}

impl Default for RootProps {
    fn default() -> Self {
        Self {
            name: "Ferris the 🦀".to_string(),
        }
    }
}

fn app(cx: Scope<RootProps>) -> Element {
    cx.render(rsx! {
        h1 { "Hello, {cx.props.name} !" }
    })
}
