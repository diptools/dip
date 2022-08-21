use crate::{channel::*, component::*, global_state::*};
use bevy::ecs::prelude::*;

pub fn handle_core_cmd(
    mut events: EventReader<CoreCommand>,
    mut create_todo: EventWriter<CreateTodo>,
    mut remove_todo: EventWriter<RemoveTodo>,
) {
    for cmd in events.iter() {
        match cmd {
            CoreCommand::CreateTodo(event) => {
                create_todo.send(event.clone());
            }
            CoreCommand::RemoveTodo(event) => {
                remove_todo.send(event.clone());
            }
        }
    }
}

pub fn update_ui_todo_list(
    mut events: EventReader<UpdateUiTodoList>,
    query: Query<(Entity, &Title, Option<&DoneAt>, &Timestamp), With<Todo>>,
    mut global_state: EventWriter<GlobalState>,
) {
    for _ in events.iter() {
        let mut todo_list = vec![];
        for (entity, title, done_at, timestamp) in query.iter() {
            let todo = UiTodo::from((entity, title, done_at, timestamp));
            todo_list.push(todo);
        }

        global_state.send(GlobalState::TodoList(todo_list));
    }
}

pub fn log_ui_todo_list(
    mut events: EventReader<UpdateUiTodoList>,
    query: Query<(Entity, &Title, Option<&DoneAt>, &Timestamp), With<Todo>>,
) {
    for _ in events.iter() {
        let mut todo_list = vec![];
        for q in query.iter() {
            let todo = UiTodo::from(q);
            todo_list.push(todo);
        }

        println!("{:#?}", todo_list);
    }
}

pub fn create_todo(
    mut events: EventReader<CreateTodo>,
    mut commands: Commands,
    mut update_ui_todo_list: EventWriter<UpdateUiTodoList>,
) {
    for e in events.iter() {
        commands.spawn_bundle(TodoBundle::from(e.title.clone()));
        update_ui_todo_list.send(UpdateUiTodoList);
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

pub fn update_done(
    mut events: EventReader<UpdateDone>,
    query: Query<(Entity, Option<&DoneAt>), With<Todo>>,
    mut update_todo_meta: EventWriter<UpdateTodoMeta>,
    mut commands: Commands,
) {
    for e in events.iter() {
        for (entity, done_at) in query.iter() {
            if e.entity == entity {
                if done_at.is_none() {
                    commands.entity(entity).insert(DoneAt::default());
                    update_todo_meta.send(UpdateTodoMeta { entity })
                } else {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

pub fn update_todo_meta(
    mut events: EventReader<UpdateTodoMeta>,
    mut query: Query<(Entity, &mut Timestamp), With<Todo>>,
    mut update_ui_todo_list: EventWriter<UpdateUiTodoList>,
) {
    for e in events.iter() {
        for (entity, mut timestamp) in query.iter_mut() {
            if e.entity == entity {
                timestamp.update();
                update_ui_todo_list.send(UpdateUiTodoList);
            }
        }
    }
}

pub fn remove_todo(
    mut events: EventReader<RemoveTodo>,
    mut commands: Commands,
    mut update_ui_todo_list: EventWriter<UpdateUiTodoList>,
) {
    for e in events.iter() {
        commands.entity(e.entity).despawn();
        update_ui_todo_list.send(UpdateUiTodoList);
    }
}
