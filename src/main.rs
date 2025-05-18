#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

mod backend;
mod components;

use components::home::Home;

fn main() {
    launch(App);
}

#[component]
fn App()->Element {
    rsx!(
        Router::<Route> {}
    )
}

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home {}
}

#[derive(Debug,Clone,PartialEq,Serialize,Deserialize)]
pub struct ToDo {
    pub id: i64,
    pub content: String,
    pub completed: bool,
}

