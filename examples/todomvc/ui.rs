use crate::ui_state::*;
use bevy_dioxus::desktop::prelude::*;

#[allow(non_snake_case)]
pub fn Root(cx: Scope) -> Element {
    let window = use_window::<UiAction>(&cx);

    let todo_list = use_read(&cx, TODO_LIST);
    let filter = use_read(&cx, FILTER);

    let new_todo = use_state(&cx, || "".to_string());
    let hovered = use_state(&cx, || None::<Entity>);

    cx.render(rsx! {
        main {
            style: "display: flex; flex-direction: column; align-items: center;",
            h1 { "todos" }

            div {
                style: "display: flex; flex-direction: column; align-items: center;",
                div {
                    style: "display: flex;",
                    label {
                        style: "margin-right: 0.25rem;",
                        r#for: "filter-select",
                        "Choose a filter:"
                    }
                    select {
                        value: format_args!("{:?}", filter),
                        onchange: |e| {
                            match e.value.as_str() {
                                "All" => { window.send(UiAction::filter_all()) }
                                "Active" => { window.send(UiAction::filter_active()) }
                                "Completed" => { window.send(UiAction::filter_completed()) }
                                _ => {}
                            }
                        },
                        option {
                            value: "All",
                            "All"
                        }
                        option {
                            value: "Active",
                            "Active"
                        }
                        option {
                            value: "Completed",
                            "Completed"
                        }
                    }
                }

                ul {
                    style: "display: flex; list-style-type: none; padding: 0;",
                    li {
                        button {
                            onclick: |_| {
                                window.send(UiAction::toggle_all());
                            },
                            "Toggle all"
                        }
                        button {
                            onclick: |_| {
                                window.send(UiAction::clear_completed());
                            },
                            "Clear completed"
                        }
                    }
                }
            }

            input {
                value: "{new_todo}",
                oninput: |e| {
                    new_todo.set(e.value.clone());
                },
                onchange: |e| {
                    window.send(UiAction::create_todo(&e.value));
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
                        key: "{todo.entity:?}",
                        style: "display: flex; align-items: center; justify-content: space-between; background: #ddd; padding: 1rem; height: 32px;",
                        onmouseover: |_| {
                            hovered.set(Some(todo.entity));
                        },
                        div { style: "display: flex; align-items: center;",
                            div {
                                style: "padding-right: 1rem;",
                                onclick: |_| {
                                    window.send(UiAction::toggle_done(&todo.entity));
                                },
                                [format_args!("{}", if todo.done_at.is_some() { "✅" } else { "❎" })],
                            }
                            input {
                                value: "{todo.title}",
                                oninput: |e| {
                                    window.send(UiAction::change_title(&todo.entity, &e.value));
                                }
                            }
                        }

                        if let Some(entity) = hovered.get() {
                            if entity == &todo.entity {
                                cx.render(rsx! {
                                    button {
                                        style: "align-self: flex-end",
                                        onclick: |_| {
                                            window.send(UiAction::remove_todo(&todo.entity));
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
    })
}
