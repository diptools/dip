use bevy::{log::LogPlugin, prelude::*};
use bevy_dioxus::desktop::prelude::*;
use dioxus::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Counter".to_string(),
            ..Default::default()
        })
        .add_plugin(LogPlugin)
        .add_plugin(DioxusPlugin::<CoreCommand, UiCommand>::new(app))
        .add_startup_system(spawn_count)
        .add_system(handle_core_cmd)
        .add_system_to_stage(CoreStage::PostUpdate, notify_counter_change)
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

// App Component
fn app(cx: Scope) -> Element {
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

// Systems
fn spawn_count(mut commands: Commands) {
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
                info!("🧠 Decrement");
                if count.0 > 0 {
                    count.0 -= 1;
                }
            }
            CoreCommand::Reset => {
                info!("🧠 Reset");
                if count.0 != 0 {
                    count.0 = 0;
                }
            }
        }
    }
}
