mod component;
mod event;
mod system;
mod ui;
mod ui_state;

use crate::{event::*, system::*, ui::Root, ui_state::*};
use dip::prelude::*;

pub struct TodoMVCPlugin;

impl Plugin for TodoMVCPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DesktopPlugin::<UiState, UiAction, NoAsyncAction>::new(Root))
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
            .add_system_to_stage(DipStage::Prepare, new_ui_todo_list);
    }
}
