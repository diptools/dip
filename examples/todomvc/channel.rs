use crate::event::*;
use bevy::ecs::prelude::*;

#[derive(Clone, Debug)]
pub enum CoreCommand {
    CreateTodo(CreateTodo),
    ToggleDone(ToggleDone),
    RemoveTodo(RemoveTodo),
}

impl CoreCommand {
    pub fn create_todo(title: &String) -> Self {
        Self::CreateTodo(CreateTodo {
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
}
