use bevy::ecs::prelude::*;

#[derive(Clone, Debug)]
pub enum CoreCommand {
    CreateTodo(CreateTodo),
}

impl CoreCommand {
    pub fn create(title: &String) -> Self {
        Self::CreateTodo(CreateTodo {
            title: title.clone(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct CreateTodo {
    pub title: String,
}

pub struct ChangeTitle {
    pub entity: Entity,
    pub title: String,
}

pub struct UpdateDone {
    pub entity: Entity,
}

pub struct UpdateTodoMeta {
    pub entity: Entity,
}

pub struct UpdateUiTodoList;
