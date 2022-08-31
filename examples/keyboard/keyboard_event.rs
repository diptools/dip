use bevy_dioxus::{
    bevy::{input::keyboard::KeyboardInput, log::LogPlugin},
    desktop::prelude::*,
};

fn main() {
    App::new()
        .insert_non_send_resource(DioxusSettings::<()> {
            keyboard_event: true,
            ..Default::default()
        })
        .add_plugin(DioxusPlugin::<GlobalState, UiAction>::new(Root))
        .add_plugin(GlobalStatePlugin)
        .init_resource::<EventType>()
        .add_plugin(LogPlugin)
        .add_system(handle_ui_action)
        .add_system(apply_global_state)
        .add_system(log_keyboard_event)
        .run();
}

#[global_state]
struct GlobalState {
    event_type: EventType,
    input_result: InputResult,
}

// Bevy Components
#[derive(Component, Debug, Clone, Default)]
struct SelectedType(EventType);

// UI -> ECS
#[derive(Debug, Clone)]
enum UiAction {
    EventType(EventType),
}

impl UiAction {
    fn keyboard_event() -> Self {
        Self::EventType(EventType::KeyboardEvent)
    }

    fn keyboard_input() -> Self {
        Self::EventType(EventType::KeyboardInput)
    }

    fn received_char() -> Self {
        Self::EventType(EventType::ReceivedCharacter)
    }
}

#[derive(Clone, Debug)]
pub enum InputResult {
    KeyboardEvent(KeyboardEvent),
    KeyboardInput(KeyboardInput),
    ReceivedCharacter(ReceivedCharacter),
    None,
}

impl Default for InputResult {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug)]
pub enum EventType {
    KeyboardEvent,
    KeyboardInput,
    ReceivedCharacter,
}

impl Default for EventType {
    fn default() -> Self {
        Self::KeyboardEvent
    }
}

// UI Component
#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let event_type = use_read(&cx, EVENT_TYPE);
    let input_result = use_read(&cx, INPUT_RESULT);
    let window = use_window::<UiAction>(&cx);

    cx.render(rsx! {
        h1 { "Keyboard Event Example" }
        p { "💡 Type any keys and checkout console. (TODO: You might need to click screen to focus.)" }

        div {
            select {
                value: format_args!("{:?}", event_type),
                onchange: |e| {
                    match e.value.as_str() {
                        "KeyboardEvent" => {
                            window.send(UiAction::keyboard_event());
                        },
                        "KeyboardInput" => {
                            window.send(UiAction::keyboard_input());
                        },
                        "ReceivedCharacter" => {
                            window.send(UiAction::received_char());
                        }
                        _ => {}
                    };
                },

                option {
                    value: "KeyboardEvent",
                    "KeyboardEvent"
                }
                option {
                    value: "KeyboardInput",
                    "KeyboardInput"
                }
                option {
                    value: "ReceivedCharacter",
                    "ReceivedCharacter"
                }
            }
        }

        code {
            [format_args!("Input result: {:#?}", input_result)],
        }
    })
}

fn handle_ui_action(mut events: EventReader<UiAction>, mut event_type: ResMut<EventType>) {
    for action in events.iter() {
        match action {
            UiAction::EventType(e) => {
                info!("🧠　EventType: {:?}", e);
                *event_type = e.clone();
            }
        }
    }
}

fn apply_global_state(
    event_type: Res<EventType>,
    mut global_state: EventWriter<GlobalState>,
    mut keyboard_events: EventReader<KeyboardEvent>,
    mut keyboard_inputs: EventReader<KeyboardInput>,
    mut received_characters: EventReader<ReceivedCharacter>,
) {
    if event_type.is_changed() {
        global_state.send(GlobalState::EventType(event_type.clone()));
    }

    match event_type.clone() {
        EventType::KeyboardEvent => {
            for e in keyboard_events.iter() {
                global_state.send(GlobalState::InputResult(InputResult::KeyboardEvent(
                    e.clone(),
                )));
            }
        }
        EventType::KeyboardInput => {
            for e in keyboard_inputs.iter() {
                global_state.send(GlobalState::InputResult(InputResult::KeyboardInput(
                    e.clone(),
                )));
            }
        }
        EventType::ReceivedCharacter => {
            for e in received_characters.iter() {
                global_state.send(GlobalState::InputResult(InputResult::ReceivedCharacter(
                    e.clone(),
                )));
            }
        }
    };
}

fn log_keyboard_event(
    mut keyboard_events: EventReader<KeyboardEvent>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut received_character_events: EventReader<ReceivedCharacter>,
    event_type: Res<EventType>,
) {
    match *event_type {
        EventType::KeyboardEvent => {
            for event in keyboard_events.iter() {
                info!("🧠 {:?}", event.clone());
            }
        }
        EventType::KeyboardInput => {
            for input in keyboard_input_events.iter() {
                info!("🧠 {:?}", input.clone());
            }
        }
        EventType::ReceivedCharacter => {
            for received_char in received_character_events.iter() {
                info!("🧠 {:?}", received_char.clone());
            }
        }
    }
}
