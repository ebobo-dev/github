use sycamore::{prelude::*, suspense::Suspense};
use web_sys::window;

use crate::components::index::Index;

mod api;
mod components;

#[component]
fn App<G: Html>() -> View<G> {
    view! {
        div {
            article {
                Suspense(fallback=view! { "Loading..." }) {
                    Index { }
                }
            }
        }
    }
}

fn main() {
    if let Some(w) = window() {
        if let Some(d) = w.document() {
            d.set_title("ebobo.dev")
        }
    }

    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(App);
}
