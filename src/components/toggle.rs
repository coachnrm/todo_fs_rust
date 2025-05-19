use dioxus::prelude::*;
use crate::{backend::server_functions::update_todo, ToDo};

#[component]
pub fn Toggle(list_signal: Signal<Vec<ToDo>>, id: i64, completed: bool) -> Element {
    let style = if completed {
        "color: #f8fafc; background-color: #f97316; padding: 0.25rem; border-radius: 0.25rem;"
    } else {
        "color: #f8fafc; background-color: #84cc16; padding: 0.25rem; border-radius: 0.25rem;"
    };

    rsx!(
        button {
            style: style,
            onclick: move |_| async move {
                match update_todo(id, completed).await {
                    Ok(_) => {
                        let new_list = list_signal
                            .read()
                            .iter()
                            .map(|item| {
                                if item.id == id {
                                    ToDo {
                                        id,
                                        content: item.content.clone(),
                                        completed: !completed,
                                    }
                                } else {
                                    item.clone()
                                }
                            })
                            .collect::<Vec<ToDo>>();
                        list_signal.set(new_list);
                    }
                    Err(_) => {}
                };
            },
            if completed { "uncomplete" } else { "complete" }
        }
    )
}
