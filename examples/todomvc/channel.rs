use crate::{event::*, resource::*};
use bevy::ecs::prelude::*;

#[derive(Clone, Debug)]
pub enum CoreCommand {
    CreateTodo(CreateTodo),
    ToggleDone(ToggleDone),
    RemoveTodo(RemoveTodo),
    ChangeFilter(ChangeFilter),
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
}
