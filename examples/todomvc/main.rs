mod component;
mod event;
mod system;
mod ui;
mod ui_state;

use crate::{event::*, system::*, ui::Root, ui_state::*};
use bevy_dioxus::{bevy::log::LogPlugin, desktop::prelude::*};
use std::{fs, process::Command};

fn main() {
    // quick and dirty way to compile tailwind css on each run
    let script = "npm run todomvc:css";
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", script])
            .output()
            .expect("failed to execute process");
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(script)
            .output()
            .expect("failed to execute process");
    };

    let css = fs::read_to_string("examples/todomvc/public/main.css")
        .expect("Should have been able to read the file");

    App::new()
        .insert_non_send_resource(DioxusSettings::<NoRootProps> {
            custom_head: Some(format!("<style>{}</style>", css)),
            ..Default::default()
        })
        .add_plugin(DioxusPlugin::<UiState, UiAction>::new(Root))
        .add_plugin(LogPlugin)
        .add_plugin(UiStatePlugin)
        .add_plugin(UiActionPlugin)
        .add_event::<UpdateTodoMeta>()
        .add_event::<NewUiTodoListRequested>()
        .add_event::<NewUiTodoListReady>()
        .add_system(create_todo)
        .add_system(change_todo_title)
        .add_system(toggle_done.before(update_todo_meta))
        .add_system(update_todo_meta)
        .add_system(remove_todo)
        .add_system(toggle_all.before(toggle_done))
        .add_system(change_filter)
        .add_system(clear_completed)
        .add_system_to_stage(UiStage::Prepare, new_ui_todo_list)
        .run();
}
