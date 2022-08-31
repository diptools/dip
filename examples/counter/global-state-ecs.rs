use bevy_dioxus::{bevy::log::LogPlugin, desktop::prelude::*};

fn main() {
    App::new()
        .add_plugin(LogPlugin)
        .add_plugin(DioxusPlugin::<GlobalState, UiAction>::new(Root))
        .add_plugin(GlobalStatePlugin)
        .init_resource::<Count>()
        .add_system(handle_ui_action)
        .add_system(update_global_state)
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

#[derive(Clone, Debug)]
enum UiAction {
    Increment,
    Decrement,
    Reset,
}

fn handle_ui_action(mut events: EventReader<UiAction>, mut count: ResMut<Count>) {
    for action in events.iter() {
        match action {
            UiAction::Increment => {
                info!("🧠 Increment");
                count.value += 1;
            }
            UiAction::Decrement => {
                if count.value > 0 {
                    info!("🧠 Decrement");
                    count.value -= 1;
                }
            }
            UiAction::Reset => {
                if count.value != 0 {
                    info!("🧠 Reset");
                    count.value = 0;
                }
            }
        };
    }
}

fn update_global_state(count: Res<Count>, mut global_state: EventWriter<GlobalState>) {
    if count.is_changed() {
        global_state.send(GlobalState::Count(count.value));
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
            onclick: move |_| window.send(UiAction::Decrement),
            disabled: "{disabled}",
            "-",
        }
        button {
            onclick: move |_| window.send(UiAction::Reset),
            disabled: "{disabled}",
            "Reset"
        }
        button {
            onclick: move |_| window.send(UiAction::Increment),
            "+",
        }
    })
}
