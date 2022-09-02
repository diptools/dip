use crate::{event::*, resource::*};
use bevy_dioxus::desktop::prelude::*;

#[ui_action]
struct UiAction {
    create_todo: CreateTodo,
    change_title: ChangeTitle,
    toggle_done: ToggleDone,
    remove_todo: RemoveTodo,
    change_filter: ChangeFilter,
    clear_completed: ClearCompleted,
    toggle_all: ToggleAll,
}

#[ui_action_creator]
impl ActionCreator {
    pub fn create_todo(title: &String) -> CreateTodo {
        CreateTodo {
            title: title.clone(),
        }
    }

    pub fn change_title(entity: &Entity, title: &String) -> ChangeTitle {
        ChangeTitle {
            entity: entity.clone(),
            title: title.clone(),
        }
    }

    pub fn toggle_done(entity: &Entity) -> ToggleDone {
        ToggleDone {
            entity: entity.clone(),
        }
    }

    pub fn remove_todo(entity: &Entity) -> RemoveTodo {
        RemoveTodo {
            entity: entity.clone(),
        }
    }

    pub fn filter_all() -> ChangeFilter {
        ChangeFilter {
            filter: Filter::All,
        }
    }

    pub fn filter_active() -> ChangeFilter {
        ChangeFilter {
            filter: Filter::Active,
        }
    }

    pub fn filter_completed() -> ChangeFilter {
        ChangeFilter {
            filter: Filter::Completed,
        }
    }

    pub fn toggle_all() -> ToggleAll {
        ToggleAll
    }

    pub fn clear_completed() -> ClearCompleted {
        ClearCompleted
    }
}
