mod channel;
mod component;
mod event;
mod global_state;
mod resource;
mod system;
mod ui;

use crate::{channel::*, event::*, global_state::*, resource::*, system::*, ui::Root};
use bevy::prelude::*;
use bevy_dioxus::desktop::prelude::*;

fn main() {
    App::new()
        .add_plugin(DioxusPlugin::<GlobalState, CoreCommand, ()>::new(Root))
        .add_plugin(GlobalStatePlugin)
        .init_resource::<Settings>()
        .add_event::<CreateTodo>()
        .add_event::<ChangeTitle>()
        .add_event::<ToggleDone>()
        .add_event::<UpdateTodoMeta>()
        .add_event::<RemoveTodo>()
        .add_event::<ChangeFilter>()
        .add_event::<ClearCompleted>()
        .add_event::<NewUiTodoListRequested>()
        .add_event::<NewUiTodoListReady>()
        .add_system(handle_core_cmd)
        .add_system_to_stage(UiStage::PreUpdate, new_ui_todo_list)
        .add_system_to_stage(UiStage::Update, update_ui_settings)
        .add_system_to_stage(UiStage::Update, update_ui_todo_list)
        .add_system_to_stage(UiStage::Update, log_ui_todo_list)
        .add_system(create_todo)
        .add_system(change_todo_title)
        .add_system(toggle_done)
        .add_system(update_todo_meta)
        .add_system(remove_todo)
        .add_system(change_filter)
        .add_system(clear_completed)
        .run();
}
