use sycamore::{prelude::*, reactive::ReadSignal};
use sycamore_router::{HistoryIntegration, Router};
use web_sys::{wasm_bindgen::prelude::*, window};

use routing::*;

mod api;
mod components;
mod routing;

#[component]
pub fn App<G: Html>() -> View<G> {
    window()
        .unwrap_throw()
        .document()
        .unwrap_throw()
        .set_title("ebobo.dev");

    view! {
        Router(
            integration=HistoryIntegration::new(),
            view=|route: ReadSignal<AppRoutes>| view! {
                div(class="app") {
                    div(class="container") {
                        Switch(route=route)
                    }
                }
            }
        )
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(App);
}
