use bevy::prelude::*;
use bevy_dioxus::desktop::prelude::*;
use dioxus::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Minimum Example".to_string(),
            ..Default::default()
        })
        .add_plugin(DioxusPlugin::<(), ()>::new(app))
        .run();
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Hello, World !" }
    })
}
