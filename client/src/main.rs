use sycamore::{prelude::*, suspense::Suspense};
use web_sys::window;

use crate::components::index::Index;

mod api;
mod components;

#[component]
pub fn App<G: Html>() -> View<G> {
    window().unwrap().document().unwrap().set_title("ebobo.dev");

    view! {
        Suspense(fallback=view! { "Loading..." }) {
            Index { }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(App);
}
