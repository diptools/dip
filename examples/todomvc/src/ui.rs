use crate::ui_state::*;
use dip::prelude::*;

#[allow(non_snake_case)]
pub fn Root(cx: Scope) -> Element {
    let window = use_window::<UiAction, NoAsyncAction>(&cx);

    let todo_list = use_read(&cx, TODO_LIST);
    let filter = use_read(&cx, FILTER);

    let new_todo = use_state(&cx, || "".to_string());
    let hovered = use_state(&cx, || None::<Entity>);

    cx.render(rsx! {
        main {
            class: "w-screen h-screen flex flex-col items-center bg-background",
            header {
                class: "w-96 flex flex-col items-center",
                h1 {
                    class: "text-8xl text-accent opacity-20 font-thin p-4",
                    "todos"
                }

                input {
                    class: "w-full text-2xl p-1 pl-16",
                    value: "{new_todo}",
                    oninput: |e| {
                        new_todo.set(e.value.clone());
                    },
                    onchange: |e| {
                        window.send(UiAction::create_todo(&e.value));
                        new_todo.set("".to_string());
                    }
                }
            }

            ul {
                style: "w-96 flex flex-col items-stretch",
                // doesn't work with dioxus <= v0.2.4 but fix is already merged in master.
                onmouseleave: |_| {
                    hovered.set(None);
                },
                todo_list.iter().map(|todo| rsx! {
                    li {
                        key: "{todo.entity:?}",
                        class: "text-2xl p-1 pl-16 flex",
                        onmouseover: |_| {
                            hovered.set(Some(todo.entity));
                        },
                        div {
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

                        if let Some(entity) = hovered.get() {
                            if entity == &todo.entity {
                                cx.render(rsx! {
                                    button {
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

            div {
                div {
                    label {
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
        }
    })
}
