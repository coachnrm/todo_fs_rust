use dioxus::prelude::*;

#[component]
pub fn Home() -> Element{
    rsx!( 
        h1 { 
            style: "color: #0ea5e9; font-size: 2rem; font-weight: bold;",
            "home" }
     )
}