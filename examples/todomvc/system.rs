use crate::{channel::*, component::*, event::*, global_state::*, resource::*};
use bevy::ecs::prelude::*;

pub fn handle_core_cmd(
    mut events: EventReader<CoreCommand>,
    mut create_todo: EventWriter<CreateTodo>,
    mut toggle_done: EventWriter<ToggleDone>,
    mut remove_todo: EventWriter<RemoveTodo>,
    mut toggle_all: EventWriter<ToggleAll>,
    mut change_filter: EventWriter<ChangeFilter>,
    mut clear_completed: EventWriter<ClearCompleted>,
) {
    for cmd in events.iter() {
        match cmd {
            CoreCommand::CreateTodo(event) => {
                create_todo.send(event.clone());
            }
            CoreCommand::ToggleDone(event) => {
                toggle_done.send(event.clone());
            }
            CoreCommand::RemoveTodo(event) => {
                remove_todo.send(event.clone());
            }
            CoreCommand::ToggleAll(event) => {
                toggle_all.send(event.clone());
            }
            CoreCommand::ChangeFilter(event) => {
                change_filter.send(event.clone());
            }
            CoreCommand::ClearCompleted(event) => {
                clear_completed.send(event.clone());
            }
        }
    }
}

pub fn update_ui_settings(settings: Res<Settings>, mut global_state: EventWriter<GlobalState>) {
    if settings.is_changed() {
        global_state.send(GlobalState::Settings(settings.into_inner().clone()));
    }
}

pub fn update_ui_todo_list(
    mut events: EventReader<NewUiTodoListReady>,
    mut global_state: EventWriter<GlobalState>,
) {
    for e in events.iter() {
        global_state.send(GlobalState::TodoList(e.todo_list.clone()));
    }
}

pub fn new_ui_todo_list(
    mut events: EventReader<NewUiTodoListRequested>,
    query: Query<(Entity, &Title, Option<&DoneAt>, &Timestamp), With<Todo>>,
    mut new_ui_todo_list_ready: EventWriter<NewUiTodoListReady>,
    settings: Res<Settings>,
) {
    for _ in events.iter() {
        let mut todo_list = vec![];
        for (entity, title, done_at, timestamp) in query.iter() {
            let todo = UiTodo::from((entity, title, done_at, timestamp));
            todo_list.push(todo);
        }

        match settings.filter {
            Filter::All => {}
            Filter::Active => {
                todo_list.retain(|todo| todo.done_at.is_none());
            }
            Filter::Completed => {
                todo_list.retain(|todo| todo.done_at.is_some());
            }
        }

        todo_list.sort_by_key(|todo| todo.created_at);

        new_ui_todo_list_ready.send(NewUiTodoListReady { todo_list });
    }
}

pub fn log_ui_todo_list(mut events: EventReader<NewUiTodoListReady>) {
    for e in events.iter() {
        println!("{:#?}", e.todo_list);
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

pub fn toggle_all(mut events: EventReader<ToggleAll>) {
    for _ in events.iter() {
        todo!("toggle_all system");
    }
}

pub fn change_filter(
    mut events: EventReader<ChangeFilter>,
    mut settings: ResMut<Settings>,
    mut new_ui_todo_list_requested: EventWriter<NewUiTodoListRequested>,
) {
    for e in events.iter() {
        settings.filter = e.filter.clone();
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
