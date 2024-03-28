use sycamore::{prelude::*, suspense::Suspense};

use crate::components::{auth::Auth, fight::Fight, footer::Footer, index::Index};

mod api;
mod components;

#[component]
fn App<G: Html>() -> View<G> {
    view! {
        div {
            Suspense(fallback=view! { "Loading..." }) {
                // Auth { }
                Index { }
            }
            Fight { }
            Footer { }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(App);
}
