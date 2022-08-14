use bevy::{log::LogPlugin, prelude::*, time::TimePlugin, window::PresentMode};
use bevy_dioxus::desktop::prelude::*;
use dioxus::prelude::*;

/// This example illustrates how to customize the default window settings
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Window Settings".to_string(),
            width: 500.,
            height: 300.,
            present_mode: PresentMode::Fifo,
            ..Default::default()
        })
        .add_plugin(LogPlugin)
        .add_plugin(TimePlugin)
        .add_plugin(DioxusPlugin::<EmptyGlobalState, (), ()>::new(Root))
        .add_system(change_title)
        .add_system(toggle_cursor)
        // .add_system(cycle_cursor_icon)
        .run();
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Window Settings" }
    })
}

/// This system will then change the title during execution
fn change_title(time: Res<Time>, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    window.set_title(format!(
        "Seconds since startup: {}",
        time.seconds_since_startup().round()
    ));
}

/// This system toggles the cursor's visibility when the space bar is pressed
fn toggle_cursor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    if input.just_pressed(KeyCode::Space) {
        window.set_cursor_lock_mode(!window.cursor_locked());
        window.set_cursor_visibility(!window.cursor_visible());
    }
}

// Todo: emit mouse input event from JS, converted it into Input
// /// This system cycles the cursor's icon through a small set of icons when clicking
// fn cycle_cursor_icon(
//     input: Res<Input<MouseButton>>,
//     mut windows: ResMut<Windows>,
//     mut index: Local<usize>,
// ) {
//     const ICONS: &[CursorIcon] = &[
//         CursorIcon::Default,
//         CursorIcon::Hand,
//         CursorIcon::Wait,
//         CursorIcon::Text,
//         CursorIcon::Copy,
//     ];
//     let window = windows.primary_mut();
//     if input.just_pressed(MouseButton::Left) {
//         println!("mouse left pressed");
//         *index = (*index + 1) % ICONS.len();
//         window.set_cursor_icon(ICONS[*index]);
//     } else if input.just_pressed(MouseButton::Right) {
//         println!("mouse right pressed");
//         *index = (*index + 1) % ICONS.len();
//         *index = if *index == 0 {
//             ICONS.len() - 1
//         } else {
//             *index - 1
//         };
//         window.set_cursor_icon(ICONS[*index]);
//     }
// }
