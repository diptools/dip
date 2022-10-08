use dip::prelude::*;

fn main() {
    App::new()
        .add_plugin(DesktopPlugin::<UiState, UiAction, NoAsyncAction>::new(Root))
        .add_plugin(UiStatePlugin)
        .add_plugin(UiActionPlugin)
        .add_system(update_name)
        .run();
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let name = use_read(&cx, NAME);
    let window = use_window::<UiAction, NoAsyncAction>(&cx);

    cx.render(rsx! {
        h1 { "Hello, {name.value} !" }

        input {
            value: "{name.value}",
            oninput: |e| {
                window.send(UiAction::update_name(e.value.to_string()));
            },
        }
    })
}

#[ui_state]
struct UiState {
    name: Name,
}

#[derive(Clone, Debug)]
pub struct Name {
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
