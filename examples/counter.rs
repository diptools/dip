use bevy_dioxus::{bevy::log::LogPlugin, desktop::prelude::*};

fn main() {
    App::new()
        .add_plugin(DioxusPlugin::<UiState, UiAction>::new(Root))
        .add_plugin(UiStatePlugin)
        .add_plugin(UiActionPlugin)
        .add_plugin(LogPlugin)
        .add_system(handle_increment)
        .add_system(handle_decrement)
        .add_system(handle_reset)
        .run();
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let count = use_read(&cx, COUNT);
    let disabled = count.value == 0;

    let window = use_window::<UiAction>(&cx);

    cx.render(rsx! {
        h1 { "Counter Example" }
        p { "count: {count.value}" }
        button {
            onclick: move |_| window.send(UiAction::decrement()),
            disabled: "{disabled}",
            "-",
        }
        button {
            onclick: move |_| window.send(UiAction::reset()),
            disabled: "{disabled}",
            "Reset"
        }
        button {
            onclick: move |_| window.send(UiAction::increment()),
            "+",
        }
    })
}

#[ui_state]
struct UiState {
    count: Count,
}

#[derive(Clone, Debug, Default)]
pub struct Count {
    value: u32,
}

#[ui_action]
impl ActionCreator {
    fn increment() -> Increment {
        Increment
    }

    fn decrement() -> Decrement {
        Decrement
    }

    fn reset() -> Reset {
        Reset
    }
}

#[derive(Clone, Debug)]
pub struct Increment;

#[derive(Clone, Debug)]
pub struct Decrement;

#[derive(Clone, Debug)]
pub struct Reset;

fn handle_increment(mut events: EventReader<Increment>, mut count: ResMut<Count>) {
    for _ in events.iter() {
        info!("🧠 Increment");
        count.value += 1;
    }
}

fn handle_decrement(mut events: EventReader<Decrement>, mut count: ResMut<Count>) {
    for _ in events.iter() {
        info!("🧠 Decrement");
        count.value -= 1;
    }
}

fn handle_reset(mut events: EventReader<Reset>, mut count: ResMut<Count>) {
    for _ in events.iter() {
        info!("🧠 Reset");
        count.value = 0;
    }
}
