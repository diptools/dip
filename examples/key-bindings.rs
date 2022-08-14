use bevy::{
    input::keyboard::KeyCode,
    log::LogPlugin,
    prelude::*,
    time::TimePlugin,
    window::{WindowCloseRequested, WindowId},
};
use bevy_dioxus::desktop::prelude::*;
use dioxus::prelude::*;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .insert_non_send_resource(DioxusSettings::<()> {
            keyboard_event: true,
            ..Default::default()
        })
        .insert_resource(WindowDescriptor {
            title: "Key Bindings".to_string(),
            ..Default::default()
        })
        .add_plugin(LogPlugin)
        .add_plugin(TimePlugin)
        .add_plugin(DioxusPlugin::<EmptyGlobalState, (), ()>::new(Root))
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

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Key Bindings Example" }
        p { "💡 Press \"Ecs\" or \"Ctrl-C\" to close window. (TODO: You might need to click screen to focus.)" }
    })
}

fn setup(mut commands: Commands) {
    let mut input_map = InputMap::new([(KeyCode::Escape, Action::CloseWindow)]);
    input_map.insert_chord([KeyCode::LControl, KeyCode::C], Action::CloseWindow);
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
