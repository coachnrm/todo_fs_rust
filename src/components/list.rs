use dioxus::prelude::*;
use crate::{components::{remove::Remove, toggle::Toggle}, Route, ToDo}; 

#[component]
pub fn List() -> Element {
    let list = use_context::<Signal<Vec<ToDo>>>();

    if list.read().is_empty() {
        rsx!(
            div {
                style: "text-align: center; font-size: 1.25rem;",
                "No ToDos"
            }
        )
    } else {
        rsx!(
            div {
                style: "height: 100vh; width: 100%; display: flex; justify-content: center;",
                ul {
                    style: "width: 83.3333%;", // Originally w-5/6
                    {list.read().iter().map(|item| {
                        let id = item.id;
                        let completed = item.completed;
                        rsx!(
                            li {
                                key: "{id}",
                                style: "
                                    margin-bottom: 0.5rem;
                                    display: flex;
                                    justify-content: space-between;
                                    align-items: center;
                                    background-color: #f1f5f9;
                                    padding: 0.5rem;
                                    border-radius: 0.25rem;
                                ",
                                div {
                                    style: format_args!(
                                        "display: flex;{}",
                                        if completed {
                                            " text-decoration: line-through;"
                                        } else {
                                            ""
                                        }
                                    ),
                                    Link { to: Route::Todo { id }, "{item.content.clone()}" }
                                }
                                div {
                                    Toggle { list_signal: list, id, completed }
                                    Remove { list_signal: list, id }
                                  }
                            }
                        )
                    })}
                }
            }
        )
    }
}
