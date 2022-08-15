mod channel;
mod component;
mod global_state;
mod system;
mod ui;

use crate::{channel::*, global_state::*, system::*, ui::Root};
use bevy::prelude::*;
use bevy_dioxus::desktop::prelude::*;

fn main() {
    App::new()
        .add_plugin(GlobalStatePlugin)
        .add_plugin(DioxusPlugin::<GlobalState, CoreCommand, ()>::new(Root))
        .add_system(handle_core_cmd)
        .add_system_to_stage(UiStage::Update, log_ui_todo_list)
        .add_system(create_todo)
        .add_system(change_todo_title)
        .add_system(update_done)
        .add_system(update_todo_meta)
        .add_system(remove_todo)
        .run();
}
