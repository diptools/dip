use crate::{channel::CoreCommand, global_state::TODO_LIST};
use bevy_dioxus::desktop::prelude::*;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Root(cx: Scope) -> Element {
    let todo_list = use_read(&cx, TODO_LIST);
    let window = use_window::<CoreCommand, ()>(&cx);

    let new_todo = use_state(&cx, || "".to_string());

    cx.render(rsx! {
        h1 { "todos" }

        todo_list.iter().map(|todo| rsx! {
            div { "{todo.title}" }
        })

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
    })
}
