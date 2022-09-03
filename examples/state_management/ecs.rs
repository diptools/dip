use bevy_dioxus::desktop::prelude::*;

fn main() {
    App::new()
        .add_plugin(DioxusPlugin::<UiState, UiAction>::new(Root))
        .add_plugin(UiStatePlugin)
        .add_plugin(UiActionPlugin)
        .init_resource::<Name>()
        .add_system(update_name)
        .add_system_to_stage(UiStage::Prepare, update_ui_state)
        .run();
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let name = use_read(&cx, NAME);
    let window = use_window::<UiAction>(&cx);

    cx.render(rsx! {
        h1 { "Hello, {name} !" }

        input {
            value: "{name}",
            oninput: |e| {
                window.send(UiAction::update_name(e.value.to_string()));
            },
        }
    })
}

#[ui_state]
struct UiState {
    name: String,
}

struct Name {
    value: String,
}

impl Default for Name {
    fn default() -> Self {
        Self {
            value: "world".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UpdateName {
    value: String,
}

#[ui_action]
impl ActionCreator {
    fn update_name(value: String) -> UpdateName {
        UpdateName { value }
    }
}

fn update_name(mut events: EventReader<UpdateName>, mut name: ResMut<Name>) {
    for action in events.iter() {
        name.value = action.value.clone();
    }
}

fn update_ui_state(name: Res<Name>, mut ui_state: EventWriter<UiState>) {
    if name.is_changed() {
        ui_state.send(UiState::Name(name.value.clone()))
    }
}
