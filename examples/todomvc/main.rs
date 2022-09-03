mod component;
mod event;
mod resource;
mod system;
mod ui;
mod ui_state;

use crate::{event::*, resource::*, system::*, ui::Root, ui_state::*};
use bevy_dioxus::{bevy::log::LogPlugin, desktop::prelude::*};

fn main() {
    App::new()
        .add_plugin(DioxusPlugin::<UiState, UiAction>::new(Root))
        .add_plugin(LogPlugin)
        .add_plugin(UiStatePlugin)
        .add_plugin(UiActionPlugin)
        .init_resource::<Settings>()
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
        .add_system_to_stage(UiStage::Prepare, update_ui_settings.after(new_ui_todo_list))
        .add_system_to_stage(
            UiStage::Prepare,
            update_ui_todo_list.after(new_ui_todo_list),
        )
        .add_system_to_stage(UiStage::Render, log_ui_todo_list)
        .run();
}
