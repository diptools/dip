use crate::{component::*, event::*, ui_state::*};
use dip::desktop::prelude::*;

pub fn new_ui_todo_list(
    mut events: EventReader<NewUiTodoListRequested>,
    query: Query<(Entity, &Title, Option<&DoneAt>, &Timestamp), With<Todo>>,
    mut ui_todo_list: ResMut<Vec<UiTodo>>,
    filter: Res<Filter>,
) {
    for _ in events.iter() {
        let mut todo_list = vec![];
        for (entity, title, done_at, timestamp) in query.iter() {
            let todo = UiTodo::from((entity, title, done_at, timestamp));
            todo_list.push(todo);
        }

        match *filter {
            Filter::All => {}
            Filter::Active => {
                todo_list.retain(|todo| todo.done_at.is_none());
            }
            Filter::Completed => {
                todo_list.retain(|todo| todo.done_at.is_some());
            }
        }

        todo_list.sort_by_key(|todo| todo.created_at);

        *ui_todo_list = todo_list;
    }
}

pub fn create_todo(
    mut events: EventReader<CreateTodo>,
    mut commands: Commands,
    mut new_ui_todo_list_requested: EventWriter<NewUiTodoListRequested>,
) {
    for e in events.iter() {
        commands.spawn_bundle(TodoBundle::from(e.title.clone()));
        new_ui_todo_list_requested.send(NewUiTodoListRequested);
    }
}

pub fn change_todo_title(
    mut events: EventReader<ChangeTitle>,
    mut query: Query<(Entity, &mut Title), With<Todo>>,
    mut update_todo_meta: EventWriter<UpdateTodoMeta>,
) {
    for e in events.iter() {
        for (entity, mut title) in query.iter_mut() {
            if e.entity == entity {
                title.value = e.title.clone();
                update_todo_meta.send(UpdateTodoMeta { entity });
            }
        }
    }
}

pub fn toggle_done(
    mut events: EventReader<ToggleDone>,
    query: Query<(Entity, Option<&DoneAt>), With<Todo>>,
    mut update_todo_meta: EventWriter<UpdateTodoMeta>,
    mut commands: Commands,
) {
    for e in events.iter() {
        for (entity, done_at) in query.iter() {
            if e.entity == entity {
                if done_at.is_none() {
                    commands.entity(entity).insert(DoneAt::default());
                } else {
                    commands.entity(entity).remove::<DoneAt>();
                }
                update_todo_meta.send(UpdateTodoMeta { entity })
            }
        }
    }
}

pub fn update_todo_meta(
    mut events: EventReader<UpdateTodoMeta>,
    mut query: Query<(Entity, &mut Timestamp), With<Todo>>,
    mut new_ui_todo_list_requested: EventWriter<NewUiTodoListRequested>,
) {
    for e in events.iter() {
        for (entity, mut timestamp) in query.iter_mut() {
            if e.entity == entity {
                timestamp.update();
                new_ui_todo_list_requested.send(NewUiTodoListRequested);
            }
        }
    }
}

pub fn remove_todo(
    mut events: EventReader<RemoveTodo>,
    mut commands: Commands,
    mut new_ui_todo_list_requested: EventWriter<NewUiTodoListRequested>,
) {
    for e in events.iter() {
        commands.entity(e.entity).despawn();
        new_ui_todo_list_requested.send(NewUiTodoListRequested);
    }
}

pub fn toggle_all(
    mut events: EventReader<ToggleAll>,
    query: Query<(Entity, Option<&DoneAt>), With<Todo>>,
    mut toggle_done: EventWriter<ToggleDone>,
) {
    for _ in events.iter() {
        let mut active_entities = vec![];
        let mut completed_entities = vec![];
        for (entity, done_at) in query.iter() {
            if done_at.is_none() {
                active_entities.push(entity);
            } else {
                completed_entities.push(entity)
            }
        }

        for entity in active_entities.iter() {
            toggle_done.send(ToggleDone {
                entity: entity.clone(),
            });
        }

        if active_entities.is_empty() || completed_entities.is_empty() {
            for entity in completed_entities.iter() {
                toggle_done.send(ToggleDone {
                    entity: entity.clone(),
                });
            }
        }
    }
}

pub fn change_filter(
    mut events: EventReader<ChangeFilter>,
    mut filter: ResMut<Filter>,
    mut new_ui_todo_list_requested: EventWriter<NewUiTodoListRequested>,
) {
    for e in events.iter() {
        *filter = e.filter.clone();
        new_ui_todo_list_requested.send(NewUiTodoListRequested);
    }
}

pub fn clear_completed(
    mut events: EventReader<ClearCompleted>,
    query: Query<(Entity, Option<&DoneAt>), With<Todo>>,
    mut commands: Commands,
    mut new_ui_todo_list_requested: EventWriter<NewUiTodoListRequested>,
) {
    for _ in events.iter() {
        for (entity, done_at) in query.iter() {
            if done_at.is_some() {
                commands.entity(entity).despawn();
                new_ui_todo_list_requested.send(NewUiTodoListRequested);
            }
        }
    }
}
