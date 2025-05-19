use dioxus::prelude::*;
use crate::{backend::server_functions::remove_todo, ToDo};

#[component]
pub fn Remove(list_signal: Signal<Vec<ToDo>>, id: i64) -> Element {
    rsx!(
        button {
            style: "
                color: #f8fafc;
                background-color: #f43f5e;
                border-radius: 0.25rem;
                padding: 0.25rem;
                margin-left: 0.25rem;
            ",
            onclick: move |_| async move {
                match remove_todo(id).await {
                    Ok(_) => {
                        let new_list = list_signal
                            .read()
                            .iter()
                            .filter(|item| item.id != id)
                            .cloned()
                            .collect::<Vec<ToDo>>();
                        list_signal.set(new_list);
                    }
                    Err(_) => {}
                }
            },
            "remove"
        }
    )
}
