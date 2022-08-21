use crate::{channel::CoreCommand, global_state::TODO_LIST};
use bevy::ecs::prelude::*;
use bevy_dioxus::desktop::prelude::*;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Root(cx: Scope) -> Element {
    let todo_list = use_read(&cx, TODO_LIST);
    let window = use_window::<CoreCommand, ()>(&cx);

    let new_todo = use_state(&cx, || "".to_string());
    let hovered = use_state(&cx, || None::<Entity>);

    cx.render(rsx! {
        h1 { "todos" }

        input {
            value: "{new_todo}",
            oninput: |e| {
                new_todo.set(e.value.clone());
            },
            onchange: |e| {
                window.send(CoreCommand::create(&e.value));
                new_todo.set("".to_string());
            }
        }

        ul {
            // doesn't work with dioxus <= v0.2.4 but fix is already merged in master.
            onmouseleave: |_| {
                hovered.set(None);
            },
            todo_list.iter().map(|todo| rsx! {
                li {
                    onmouseover: |_| {
                        hovered.set(Some(todo.entity));
                    },
                    div {
                        "{todo.title}"
                    }

                    if let Some(entity) = hovered.get() {
                        if entity == &todo.entity {
                            cx.render(rsx! {
                                div {
                                    "hovered"
                                }
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            })
        }
    })
}
