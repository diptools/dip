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

// Action Creator
impl UiAction {
    pub fn create_todo(title: &String) -> Self {
        Self::CreateTodo(CreateTodo {
            title: title.clone(),
        })
    }

    pub fn change_title(entity: &Entity, title: &String) -> Self {
        Self::ChangeTitle(ChangeTitle {
            entity: entity.clone(),
            title: title.clone(),
        })
    }

    pub fn toggle_done(entity: &Entity) -> Self {
        Self::ToggleDone(ToggleDone {
            entity: entity.clone(),
        })
    }

    pub fn remove_todo(entity: &Entity) -> Self {
        Self::RemoveTodo(RemoveTodo {
            entity: entity.clone(),
        })
    }

    pub fn filter_all() -> Self {
        Self::ChangeFilter(ChangeFilter {
            filter: Filter::All,
        })
    }

    pub fn filter_active() -> Self {
        Self::ChangeFilter(ChangeFilter {
            filter: Filter::Active,
        })
    }

    pub fn filter_completed() -> Self {
        Self::ChangeFilter(ChangeFilter {
            filter: Filter::Completed,
        })
    }

    pub fn toggle_all() -> Self {
        Self::ToggleAll(ToggleAll)
    }

    pub fn clear_completed() -> Self {
        Self::ClearCompleted(ClearCompleted)
    }
}
