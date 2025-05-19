use dioxus::prelude::*;
use crate::{backend::server_functions::add_new_todo, ToDo};

#[component]
pub fn Add() -> Element {
    let mut content = use_signal(|| String::new());
    let mut list_signal = use_context::<Signal<Vec<ToDo>>>();

    rsx!(
        div {
            style: "
                display: flex;
                width: 100%;
                justify-content: center;
                margin-top: 0.5rem;
                margin-bottom: 0.5rem;
            ",
            div {
                style: "width: 83.3333%;", // approximates w-5/6
                input {
                    style: "
                        width: 75%;
                        padding: 0.25rem;
                        border-radius: 0.25rem;
                        border-width: 2px;
                        border-style: solid;
                        border-color: #f1f5f9;
                    ",
                    r#type: "text",
                    value: content,
                    oninput: move |e| content.set(e.value()),
                }
                button {
                    style: "
                        color: #f8fafc;
                        background-color: #0ea5e9;
                        padding: 0.25rem;
                        border-radius: 0.25rem;
                        width: 25%;
                    ",
                    onclick: move |_| async move {
                        match add_new_todo((*content.read()).clone(), false).await {
                            Ok(id) => {
                                let todo = ToDo {
                                    id,
                                    content: (*content.read()).clone(),
                                    completed: false,
                                };
                                let mut new_list = list_signal.read().clone();
                                new_list.push(todo);
                                list_signal.set(new_list);
                            }
                            Err(_) => {}
                        }
                        content.set(String::new());
                    },
                    disabled: content.read().trim().is_empty(),
                    "add"
                }
            }
        }
    )
}
