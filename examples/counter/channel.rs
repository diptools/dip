use bevy::{log::LogPlugin, prelude::*};
use bevy_dioxus::desktop::prelude::*;
use dioxus::prelude::*;

fn main() {
    App::new()
        .add_plugin(LogPlugin)
        .add_plugin(DioxusPlugin::<EmptyGlobalState, CoreCommand, UiCommand>::new(Root))
        .add_startup_system(setup)
        .add_system(handle_core_cmd)
        .add_system(notify_counter_change)
        .run();
}

// Bevy Components
#[derive(Component, Default)]
struct Count(u32);

// Core <-> UI
#[derive(Clone, Debug)]
enum CoreCommand {
    Increment,
    Decrement,
    Reset,
}

#[derive(Clone, Debug)]
enum UiCommand {
    CountChanged(u32),
}

// Systems
fn setup(mut commands: Commands) {
    info!("🧠 Spawn count");
    commands.spawn().insert(Count::default());
}

fn notify_counter_change(query: Query<&Count, Changed<Count>>, mut ui: EventWriter<UiCommand>) {
    for count in query.iter() {
        info!("🧠 Counter Changed: {}", count.0);
        ui.send(UiCommand::CountChanged(count.0));
    }
}

fn handle_core_cmd(mut events: EventReader<CoreCommand>, mut query: Query<&mut Count>) {
    for cmd in events.iter() {
        let mut count = query.single_mut();
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
        }
    }
}

// UI Component
#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let window = use_window::<CoreCommand, UiCommand>(&cx);
    let count = use_state(&cx, || 0);
    let disabled = *count == 0;

    use_future(&cx, (), |_| {
        let count = count.clone();
        let rx = window.receiver();

        async move {
            while let Some(cmd) = rx.receive().await {
                match cmd {
                    UiCommand::CountChanged(c) => count.modify(|_| c),
                }
            }
        }
    });

    cx.render(rsx! {
        h1 { "Counter Example" }
        p { "count: {count}" }
        button {
            onclick: move |_| window.send(CoreCommand::Decrement),
            disabled: "{disabled}",
            "-",
        }
        button {
            onclick: move |_| window.send(CoreCommand::Reset),
            disabled: "{disabled}",
            "Reset"
        }
        button {
            onclick: move |_| window.send(CoreCommand::Increment),
            "+",
        }
    })
}
