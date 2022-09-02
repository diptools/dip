use bevy_dioxus::{bevy::log::LogPlugin, desktop::prelude::*};

/// This example open window with specific size then resize
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 500.,
            height: 300.,
            ..default()
        })
        .insert_non_send_resource(DioxusSettings::<NoRootProps> {
            keyboard_event: true,
            ..Default::default()
        })
        .add_plugin(DioxusPlugin::<NoUiState, NoUiAction>::new(Root))
        .add_plugin(LogPlugin)
        .add_system(toggle_override)
        .add_system(change_scale_factor)
        .run();
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Scale Factor Override" }
        p { "💡 Press \"Enter\" to toggle scale factor overrides when enter is pressed. (TODO: You might need to click screen to focus.)" }
        p { "Press \"Up\" or \"Down\" key to increase/decrease scale factor" }
    })
}

/// This system toggles scale factor overrides when enter is pressed
fn toggle_override(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    if input.just_pressed(KeyCode::Return) {
        window.set_scale_factor_override(window.scale_factor_override().xor(Some(1.)));
    }
}

/// This system changes the scale factor override when up or down is pressed
fn change_scale_factor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    if input.just_pressed(KeyCode::Up) {
        window.set_scale_factor_override(window.scale_factor_override().map(|n| n + 1.));
    } else if input.just_pressed(KeyCode::Down) {
        window.set_scale_factor_override(window.scale_factor_override().map(|n| (n - 1.).max(1.)));
    }
}
