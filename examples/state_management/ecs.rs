use bevy_dioxus::desktop::prelude::*;

fn main() {
    App::new()
        .add_plugin(GlobalStatePlugin)
        .add_plugin(DioxusPlugin::<GlobalState, UiAction>::new(Root))
        .add_event::<UpdateGlobalState>()
        .add_startup_system(setup)
        .add_system(handle_ui_action)
        .add_system(update_global_state)
        .run();
}

#[global_state]
struct GlobalState {
    name: String,
}

#[derive(Component, Clone, Debug)]
struct Name(String);

impl Default for Name {
    fn default() -> Self {
        Self("world".to_string())
    }
}

#[derive(Clone, Debug)]
struct UiAction(String);

struct UpdateGlobalState;

fn setup(mut commands: Commands, mut update_global_state: EventWriter<UpdateGlobalState>) {
    commands.spawn().insert(Name::default());
    update_global_state.send(UpdateGlobalState);
}

fn handle_ui_action(
    mut events: EventReader<UiAction>,
    mut query: Query<&mut Name>,
    mut update_global_state: EventWriter<UpdateGlobalState>,
) {
    for action in events.iter() {
        let mut name = query.single_mut();
        name.0 = action.0.clone();

        update_global_state.send(UpdateGlobalState);
    }
}

fn update_global_state(
    mut events: EventReader<UpdateGlobalState>,
    query: Query<&Name>,
    mut global_state: EventWriter<GlobalState>,
) {
    for _ in events.iter() {
        let name = query.single();
        global_state.send(GlobalState::Name(name.0.clone()))
    }
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
                window.send(UiAction(e.value.to_string()));
            },
        }
    })
}
