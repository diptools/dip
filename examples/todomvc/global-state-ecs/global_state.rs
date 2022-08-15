use crate::{channel::*, component::*};
use bevy::prelude::*;
use bevy_dioxus::core::prelude::*;
use chrono::{DateTime, Utc};
use dioxus::fermi::{Atom, AtomRoot};
use std::rc::Rc;

pub static TODO_LIST: Atom<Vec<UiTodo>> = |_| vec![];

// #[derive(GlobalState)]
pub struct GlobalState {
    pub todo_list: Vec<UiTodo>,
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

// Generated
pub struct GlobalStatePlugin;

impl Plugin for GlobalStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreateTodo>()
            .add_event::<ChangeTitle>()
            .add_event::<UpdateDone>()
            .add_event::<UpdateTodoMeta>()
            .add_event::<UpdateUiTodoList>();
    }
}

impl GlobalStateHandler for GlobalState {
    fn handler(self, atom_root: Rc<AtomRoot>) {}
}
