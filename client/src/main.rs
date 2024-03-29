use sycamore::{prelude::*, suspense::Suspense};

use crate::components::{fight::Fight, footer::Footer, index::Index};

mod api;
mod components;

#[component]
fn App<G: Html>() -> View<G> {
    view! {
        div {
            header {
                h1 { "ebobo.dev" }
            }

            article {
                Suspense(fallback=view! { "Loading..." }) {
                    Index { }
                }

                Fight { }
            }

            Footer { }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(App);
}
