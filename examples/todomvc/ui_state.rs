use crate::{component::*, event::*, resource::*};
use bevy_dioxus::desktop::prelude::*;
use chrono::{DateTime, Utc};

#[ui_state]
pub struct UiState {
    todo_list: Vec<UiTodo>,
    settings: Settings,
}

#[derive(Component, Clone, Debug)]
pub struct UiTodo {
    pub entity: Entity,
    pub title: String,
    pub done_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<(Entity, &Title, Option<&DoneAt>, &Timestamp)> for UiTodo {
    fn from(
        (entity, title, done_at, timestamp): (Entity, &Title, Option<&DoneAt>, &Timestamp),
    ) -> Self {
        Self {
            entity,
            title: title.value.clone(),
            done_at: match done_at {
                Some(done_at) => Some(done_at.time),
                None => None,
            },
            created_at: timestamp.created_at,
            updated_at: timestamp.updated_at,
        }
    }
}

#[ui_action]
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
