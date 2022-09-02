use bevy_dioxus::{bevy::log::LogPlugin, desktop::prelude::*};

fn main() {
    App::new()
        .add_plugin(LogPlugin)
        .add_plugin(DioxusPlugin::<GlobalState, UiAction>::new(Root))
        .add_plugin(GlobalStatePlugin)
        .add_plugin(UiActionPlugin)
        .init_resource::<Count>()
        .add_system_to_stage(UiStage::Prepare, update_global_state)
        .add_system(handle_increment)
        .add_system(handle_decrement)
        .add_system(handle_reset)
        .run();
}

#[global_state]
struct GlobalState {
    count: u32,
    disabled: bool,
}

#[derive(Clone, Debug, Default)]
struct Count {
    value: u32,
}

#[ui_action]
#[derive(Clone, Debug)]
struct UiAction {
    increment: Increment,
    decrement: Decrement,
    reset: Reset,
}

#[ui_action_creator]
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

fn update_global_state(count: Res<Count>, mut global_state: EventWriter<GlobalState>) {
    if count.is_changed() {
        global_state.send(GlobalState::Count(count.value));
    }
}

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

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let count = use_read(&cx, COUNT);
    let disabled = *count == 0;

    let window = use_window::<UiAction>(&cx);

    cx.render(rsx! {
        h1 { "Counter Example" }
        p { "count: {count}" }
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
