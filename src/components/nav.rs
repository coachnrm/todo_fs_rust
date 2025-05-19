use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Nav() -> Element {
    rsx!(
        div {
            style: "
                margin-top: 1.25rem;
                margin-bottom: 1.25rem;
                color: #f97316;
                text-align: center;
                font-size: 3rem;
                line-height: 1;
            ",
            Link { to: Route::Home {}, "To-Do List" }
        }
        Outlet::<Route> {}
    )
}
