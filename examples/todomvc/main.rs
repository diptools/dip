mod component;
mod event;
mod global_state;
mod resource;
mod system;
mod ui;
mod ui_action;

use crate::{event::*, global_state::*, resource::*, system::*, ui::Root, ui_action::*};
use bevy_dioxus::{bevy::log::LogPlugin, desktop::prelude::*};

fn main() {
    App::new()
        .add_plugin(DioxusPlugin::<GlobalState, UiAction>::new(Root))
        .add_plugin(LogPlugin)
        .add_plugin(GlobalStatePlugin)
        .add_plugin(UiActionPlugin)
        .init_resource::<Settings>()
        .add_event::<CreateTodo>()
        .add_event::<ChangeTitle>()
        .add_event::<ToggleDone>()
        .add_event::<UpdateTodoMeta>()
        .add_event::<RemoveTodo>()
        .add_event::<ToggleAll>()
        .add_event::<ChangeFilter>()
        .add_event::<ClearCompleted>()
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
