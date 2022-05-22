use bevy::{
    core::CorePlugin,
    input::keyboard::KeyCode,
    log::LogPlugin,
    prelude::*,
    window::{WindowCloseRequested, WindowId},
};
use bevy_dioxus::desktop::prelude::*;
use dioxus::prelude::*;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Key Bindings".to_string(),
            ..Default::default()
        })
        .add_plugin(LogPlugin)
        .add_plugin(DioxusPlugin::<(), ()>::new(app))
        .add_plugin(CorePlugin)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_startup_system(setup)
        .add_system(close_window)
        .run();
}

#[derive(Component)]
struct User;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    CloseWindow,
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Key Bindings Example" }
        p { "💡 Press \"Ecs\" or \"Ctrl-C\" to close window. (TODO: You might need to click screen to focus.)" }
    })
}

fn setup(mut commands: Commands) {
    let mut input_map = InputMap::new([(Action::CloseWindow, KeyCode::Escape)]);
    input_map.insert_chord(Action::CloseWindow, [KeyCode::LControl, KeyCode::C]);
    commands
        .spawn()
        .insert(User)
        .insert_bundle(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map,
        });
}

fn close_window(
    query: Query<&ActionState<Action>, With<User>>,
    mut events: EventWriter<WindowCloseRequested>,
) {
    let action_state = query.single();
    if action_state.just_pressed(Action::CloseWindow) {
        events.send(WindowCloseRequested {
            id: WindowId::primary(),
        });
    }
}
