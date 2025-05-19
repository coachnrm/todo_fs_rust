use dioxus::prelude::*;
use crate::backend::server_functions::get_single_todo;

#[component]
pub fn Todo(id: i64) -> Element {
    let mut list = use_signal(|| vec![]);
    let navigator = use_navigator();

    let _ = use_resource(move || async move {
        match get_single_todo(id).await {
            Ok(todo) => list.set(vec![todo]),
            Err(_) => ()
        }
    });

    if !list.read().is_empty() {
        rsx!(
            div {
                style: "margin-top: 1.25rem; margin-bottom: 1.25rem; display: flex; justify-content: center;",
                div {
                    style: "
                        border-style: solid;
                        border-color: #f1f5f9;
                        border-width: 2px;
                        border-radius: 0.25rem;
                        padding: 0.25rem;
                    ",
                    button {
                        style: "
                            background-color: #f8fafc;
                            border-radius: 0.25rem;
                            padding: 0.25rem;
                        ",
                        onclick: move |_| navigator.go_back(),
                        "go back"
                    }
                    div {
                        style: "font-size: 1.0rem; line-height: 1.0rem;",
                        h1 { "id: {list.read()[0].id}" }
                        h1 { "content: {list.read()[0].content}" }
                        h1 { "completed: {list.read()[0].completed}" }
                    }
                }
            }
        )
    } else {
        rsx!(
            div {
                style: "margin-top: 1.0rem; margin-bottom: 1.0rem; display: flex; justify-content: center;",
                button {
                    style: "
                        background-color: #f8fafc;
                        border-radius: 0.25rem;
                        padding: 0.25rem;
                    ",
                    onclick: move |_| navigator.go_back(),
                    "go back"
                }
                div {
                    style: "font-size: 1.0rem; line-height: 1;",
                    "To-Do id : {id} Not Found!"
                }
            }
        )
    }
}
