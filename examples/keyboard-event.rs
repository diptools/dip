use bevy::{input::keyboard::KeyboardInput, log::LogPlugin, prelude::*};
use bevy_dioxus::desktop::prelude::*;
use dioxus::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Counter".to_string(),
            ..Default::default()
        })
        .add_plugin(LogPlugin)
        .add_plugin(DioxusPlugin::<CoreCommand, ()>::new(app, ()))
        .add_startup_system(setup)
        .add_system(handle_core_cmd)
        .add_system(log_keyboard_event)
        .run();
}

// Bevy Components
#[derive(Component, Debug, Clone, Default)]
struct SelectedType(EventType);

// UI -> Core
#[derive(Debug, Clone)]
enum CoreCommand {
    NewEventType(EventType),
}

#[derive(Component, Debug, Clone, PartialEq)]
enum EventType {
    KeyboardEvent,
    KeyboardInput,
    ReceivedCharacter,
}

impl Default for EventType {
    fn default() -> Self {
        Self::KeyboardEvent
    }
}

// App Component
fn app(cx: Scope) -> Element {
    let event_type = use_state(&cx, || EventType::default());
    let window = use_window::<CoreCommand, ()>(&cx);

    use EventType::*;
    cx.render(rsx! {
        h1 { "Keyboard Event Example" }
        p { "Type any keys and checkout console. (TODO: You might need to click screen to focus.)" }

        div {
            input {
                r#type: "radio",
                id: "keyboard-event",
                checked: format_args!("{}", *event_type == KeyboardEvent),
                onchange: |_e| {
                    event_type.modify(|_| KeyboardEvent);
                    window.send(CoreCommand::NewEventType(KeyboardEvent));
                },
                style: "margin: 0.5rem;",
            }
            label {
                r#for: "keyboard-event",
                style: "padding-right: 1rem;",
                "KeyboardEvent",
            }
            input {
                r#type: "radio",
                id: "keyboard-input",
                checked: format_args!("{}", *event_type == KeyboardInput),
                onchange: |_e| {
                    event_type.modify(|_| KeyboardInput);
                    window.send(CoreCommand::NewEventType(KeyboardInput));
                },
                style: "margin: 0.5rem;",
            }
            label {
                r#for: "keyboard-input",
                style: "padding-right: 1rem;",
                "KeyboardInput",
            }
            input {
                r#type: "radio",
                id: "received-character",
                checked: format_args!("{}", *event_type == ReceivedCharacter),
                onchange: |_e| {
                    event_type.modify(|_| ReceivedCharacter);
                    window.send(CoreCommand::NewEventType(ReceivedCharacter));
                },
                style: "margin: 0.5rem;",
            }
            label {
                r#for: "received-character",
                style: "padding-right: 1rem;",
                "ReceivedCharacter",
            }
        }
    })
}

// Systems
fn setup(mut commands: Commands) {
    commands.spawn().insert(SelectedType::default());
}

fn handle_core_cmd(mut events: EventReader<CoreCommand>, mut query: Query<&mut SelectedType>) {
    for cmd in events.iter() {
        let mut selected = query.single_mut();
        match cmd {
            CoreCommand::NewEventType(e) => {
                info!("🧠 NewEventType: {:?}", e);
                selected.0 = e.clone();
            }
        }
    }
}

fn log_keyboard_event(
    mut keyboard_events: EventReader<KeyboardEvent>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut received_character_events: EventReader<ReceivedCharacter>,
    query: Query<&SelectedType>,
) {
    let selected = query.single();
    if selected.0 == EventType::KeyboardEvent {
        for event in keyboard_events.iter() {
            info!("🧠 {:?}", event.clone());
        }
    }

    if selected.0 == EventType::KeyboardInput {
        for input in keyboard_input_events.iter() {
            info!("🧠 {:?}", input.clone());
        }
    }

    if selected.0 == EventType::ReceivedCharacter {
        for received_char in received_character_events.iter() {
            info!("🧠 {:?}", received_char.clone());
        }
    }
}
