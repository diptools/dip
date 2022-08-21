use crate::{channel::CoreCommand, global_state::*};
use bevy::ecs::prelude::*;
use bevy_dioxus::desktop::prelude::*;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Root(cx: Scope) -> Element {
    let window = use_window::<CoreCommand, ()>(&cx);

    let todo_list = use_read(&cx, TODO_LIST);
    let settings = use_read(&cx, SETTINGS);

    let new_todo = use_state(&cx, || "".to_string());
    let hovered = use_state(&cx, || None::<Entity>);

    cx.render(rsx! {
        body {
            main {
                style: "display: flex; flex-direction: column; align-items: center;",
                h1 { "todos" }

                input {
                    value: "{new_todo}",
                    oninput: |e| {
                        new_todo.set(e.value.clone());
                    },
                    onchange: |e| {
                        window.send(CoreCommand::create_todo(&e.value));
                        new_todo.set("".to_string());
                    }
                }

                ul {
                    // doesn't work with dioxus <= v0.2.4 but fix is already merged in master.
                    style: "max-width: 400px; width: 100vw; list-style-type: none;",
                    onmouseleave: |_| {
                        hovered.set(None);
                    },
                    todo_list.iter().map(|todo| rsx! {
                        li {
                            style: "display: flex; align-items: center; justify-content: space-between; background: #ddd; padding: 1rem; height: 32px;",
                            onmouseover: |_| {
                                hovered.set(Some(todo.entity));
                            }, div { style: "display: flex; align-items: center;",
                                div {
                                    style: "padding-right: 1rem;",
                                    onclick: |_| {
                                        window.send(CoreCommand::toggle_done(&todo.entity));
                                    },
                                    [format_args!("{}", if todo.done_at.is_some() { "✅" } else { "❎" })],
                                }
                                div {
                                    "{todo.title}"
                                }
                            }

                            if let Some(entity) = hovered.get() {
                                if entity == &todo.entity {
                                    cx.render(rsx! {
                                        button {
                                            style: "align-self: flex-end",
                                            onclick: |_| {
                                                window.send(CoreCommand::remove_todo(&todo.entity));
                                            },
                                            "X"
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
            }

            footer {
                div { [format_args!("Selected: {:?}", settings.filter)] }
                ul {
                    style: "display: flex; list-style-type: none;",
                    li {
                        style: "padding: 0.25rem;",
                        onclick: |_| {
                            window.send(CoreCommand::filter_all());
                        },
                        "All"
                    }
                    li {
                        style: "padding: 0.25rem;",
                        onclick: |_| {
                            window.send(CoreCommand::filter_active());
                        },
                        "Active"
                    }
                    li {
                        style: "padding: 0.25rem;",
                        onclick: |_| {
                            window.send(CoreCommand::filter_completed());
                        },
                        "Completed"
                    }
                }
                button {
                    onclick: |_| {
                        window.send(CoreCommand::toggle_all());
                    },
                    "Toggle all"
                }
                button {
                    onclick: |_| {
                        window.send(CoreCommand::clear_completed());
                    },
                    "Clear completed"
                }
            }
        }
    })
}
