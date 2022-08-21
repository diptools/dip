use crate::component::*;
use bevy::prelude::*;
use bevy_dioxus::core::prelude::*;
use chrono::{DateTime, Utc};

// Check what is generated with cargo-expand
//
// ```sh
// cargo install expand # if not installed
// cargo expand --example todo-global-state-ecs
// ```
//
#[global_state]
struct GlobalState {
    todo_list: Vec<UiTodo>,
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
