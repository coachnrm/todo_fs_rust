#![allow(non_snake_case)]

use backend::server_functions::get_todo_list;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

mod backend;
mod components;

use components::{home::Home, nav::Nav, todo::Todo};

fn main() {
    launch(App);
}

#[component]
fn App()->Element {
    let mut list = use_signal(|| vec![]);

    let _ = use_resource(move || async move {
        match get_todo_list().await {
            Ok(todos) => list.set(todos),
            Err(_) => ()
        }
    });

    use_context_provider(|| list);
    
    rsx!(
        document::Stylesheet { href: asset!("/assets/main.css")}
        Router::<Route> {}
    )
}

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[layout(Nav)]
    #[route("/")]
    Home {},
    #[route("/todo/:id")]
    Todo {id: i64}
}

#[derive(Debug,Clone,PartialEq,Serialize,Deserialize)]
pub struct ToDo {
    pub id: i64,
    pub content: String,
    pub completed: bool,
}

