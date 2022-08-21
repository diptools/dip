use bevy::ecs::prelude::*;

#[derive(Clone, Debug)]
pub enum CoreCommand {
    CreateTodo(CreateTodo),
    RemoveTodo(RemoveTodo),
}

impl CoreCommand {
    pub fn create(title: &String) -> Self {
        Self::CreateTodo(CreateTodo {
            title: title.clone(),
        })
    }

    pub fn remove(entity: &Entity) -> Self {
        Self::RemoveTodo(RemoveTodo {
            entity: entity.clone(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Clone, Debug)]
pub struct RemoveTodo {
    pub entity: Entity,
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
