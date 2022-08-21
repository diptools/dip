use crate::{global_state::UiTodo, resource::*};
use bevy::ecs::prelude::*;

// UI -> Core

#[derive(Clone, Debug)]
pub struct CreateTodo {
    pub title: String,
}

pub struct ChangeTitle {
    pub entity: Entity,
    pub title: String,
}

#[derive(Clone, Debug)]
pub struct ToggleDone {
    pub entity: Entity,
}

#[derive(Clone, Debug)]
pub struct RemoveTodo {
    pub entity: Entity,
}

#[derive(Clone, Debug)]
pub struct ChangeFilter {
    pub filter: Filter,
}

#[derive(Clone, Debug)]
pub struct ClearCompleted;

// Internal (System -> System)

pub struct UpdateTodoMeta {
    pub entity: Entity,
}

pub struct NewUiTodoListRequested;

pub struct NewUiTodoListReady {
    pub todo_list: Vec<UiTodo>,
}
