use bevy::{log::LogPlugin, prelude::*};
use bevy_dioxus::{core::prelude::*, desktop::prelude::*};
use dioxus::prelude::*;

fn main() {
    App::new()
        .add_plugin(LogPlugin)
        .add_plugin(GlobalStatePlugin)
        .add_plugin(DioxusPlugin::<GlobalState, CoreCommand, ()>::new(Root))
        .add_startup_system(setup)
        .add_system(handle_core_cmd)
        .run();
}

/// Make sure to implement Default trait.
#[derive(Component, Clone, Debug, Default, GlobalState)]
struct Count(u32);

#[derive(Component, Clone, Debug, GlobalState)]
struct Disabled(bool);

impl Default for Disabled {
    fn default() -> Self {
        Self(true)
    }
}

/// Warning: Execution order matters here. Make sure to place this line after deriving all GlobalState.
#[derive(GlobalStatePlugin)]
struct GlobalStatePlugin;

#[derive(Clone, Debug)]
enum CoreCommand {
    Increment,
    Decrement,
    Reset,
}

fn setup(mut commands: Commands) {
    info!("🧠 Spawn count");
    commands
        .spawn()
        .insert(Count::default())
        .insert(Disabled::default());
}

fn handle_core_cmd(
    mut events: EventReader<CoreCommand>,
    mut query: Query<(&mut Count, &mut Disabled)>,
) {
    for cmd in events.iter() {
        let (mut count, mut disabled) = query.single_mut();
        match cmd {
            CoreCommand::Increment => {
                info!("🧠 Increment");
                count.0 += 1;
            }
            CoreCommand::Decrement => {
                if count.0 > 0 {
                    info!("🧠 Decrement");
                    count.0 -= 1;
                }
            }
            CoreCommand::Reset => {
                if count.0 != 0 {
                    info!("🧠 Reset");
                    count.0 = 0;
                }
            }
        };
        disabled.0 = count.0 == 0;
    }
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let count = use_read(&cx, COUNT);
    let disabled = use_read(&cx, DISABLED);

    let window = use_window::<CoreCommand, ()>(&cx);

    cx.render(rsx! {
        h1 { "Counter Example" }
        p { "count: {count.0}" }
        button {
            onclick: move |_| window.send(CoreCommand::Decrement),
            disabled: "{disabled.0}",
            "-",
        }
        button {
            onclick: move |_| window.send(CoreCommand::Reset),
            disabled: "{disabled.0}",
            "Reset"
        }
        button {
            onclick: move |_| window.send(CoreCommand::Increment),
            "+",
        }
    })
}
