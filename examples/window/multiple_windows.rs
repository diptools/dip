use bevy::{
    log::LogPlugin,
    prelude::*,
    window::{CreateWindow, WindowDescriptor, WindowId},
};
use bevy_dioxus::desktop::prelude::*;
use dioxus::prelude::*;

/// This example attemps to create a second window then warning shows up
fn main() {
    App::new()
        .add_plugin(LogPlugin)
        .add_plugin(DioxusPlugin::<EmptyGlobalState, NewWindow>::new(Root))
        .add_event::<NewWindow>()
        .add_system(create_new_window)
        .run();
}

#[derive(Clone, Debug)]
struct NewWindow;

fn create_new_window(mut events: EventReader<NewWindow>, mut create: EventWriter<CreateWindow>) {
    for _ in events.iter() {
        let id = WindowId::new();
        create.send(CreateWindow {
            id,
            descriptor: WindowDescriptor::default(),
        });
    }
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let window = use_window::<NewWindow>(&cx);

    cx.render(rsx! {
        h1 { "Multiple Windows isn't supported yet!" }
        p { "Currently Dioxus's VirtualDom doesn't support multi-windows. Please be patient." }
        button {
            onclick: |_| {
                window.send(NewWindow);
            },
            "New Window"
        }
        p { "You'll see warning log" }
    })
}
