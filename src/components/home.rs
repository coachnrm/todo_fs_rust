use dioxus::prelude::*;

use crate::components::{list::List, add::Add};

#[component]
pub fn Home() -> Element{
    rsx!( 
        Add {}
        List {}
     )
}