use bevy::prelude::*;
use bevy_dioxus::{core::prelude::*, desktop::prelude::*};
use dioxus::prelude::*;

fn main() {
    App::new()
        .add_plugin(GlobalStatePlugin)
        .add_plugin(DioxusPlugin::<GlobalState, CoreCommand, ()>::new(Root))
        .add_startup_system(setup)
        .add_system(handle_core_cmd)
        .run();
}

#[derive(Component, Clone, Debug, GlobalState)]
struct Name(String);

impl Default for Name {
    fn default() -> Self {
        Self("world".to_string())
    }
}

#[derive(GlobalStatePlugin)]
struct GlobalStatePlugin;

#[derive(Clone, Debug)]
struct CoreCommand(String);

fn setup(mut commands: Commands) {
    commands.spawn().insert(Name::default());
}

fn handle_core_cmd(mut events: EventReader<CoreCommand>, mut query: Query<&mut Name>) {
    for cmd in events.iter() {
        let mut name = query.single_mut();
        name.0 = cmd.0.clone();
    }
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let name = use_read(&cx, NAME);
    let window = use_window::<CoreCommand, ()>(&cx);

    cx.render(rsx! {
        h1 { "Hello, {name.0} !" }

        input {
            value: "{name.0}",
            oninput: |e| {
                window.send(CoreCommand(e.value.to_string()));
            },
        }
    })
}
