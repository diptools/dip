mod channel;
mod component;
mod global_state;
mod ui;

use crate::{channel::*, component::*, global_state::*, ui::Root};
use bevy::prelude::*;
use bevy_dioxus::desktop::prelude::*;
use std::convert::From;

fn main() {
    App::new()
        .add_plugin(GlobalStatePlugin)
        .add_plugin(DioxusPlugin::<GlobalState, CoreCommand, ()>::new(Root))
        .add_system(handle_core_cmd)
        .add_system_to_stage(UiStage::Update, log_ui_todo_list)
        .add_system(create_todo)
        .add_system(change_todo_title)
        .add_system(update_done)
        .add_system(update_todo_meta)
        .run();
}

// Systems
fn handle_core_cmd(mut events: EventReader<CoreCommand>, mut create_todo: EventWriter<CreateTodo>) {
    for cmd in events.iter() {
        match cmd {
            CoreCommand::CreateTodo(event) => {
                create_todo.send(event.clone());
            }
        }
    }
}

fn log_ui_todo_list(
    mut events: EventReader<UpdateUiTodoList>,
    query: Query<(Entity, &Title, Option<&DoneAt>, &Timestamp), With<Todo>>,
) {
    for _ in events.iter() {
        let mut todo_list = vec![];
        for (entity, title, done_at, timestamp) in query.iter() {
            let todo = UiTodo::from((entity, title, done_at, timestamp));
            todo_list.push(todo);
        }

        println!("{:#?}", todo_list);
    }
}

fn create_todo(
    mut events: EventReader<CreateTodo>,
    mut commands: Commands,
    mut update_ui_todo_list: EventWriter<UpdateUiTodoList>,
) {
    for e in events.iter() {
        commands.spawn_bundle(TodoBundle::from(e.title.clone()));
        update_ui_todo_list.send(UpdateUiTodoList);
    }
}

fn change_todo_title(
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

fn update_done(
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

fn update_todo_meta(
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
