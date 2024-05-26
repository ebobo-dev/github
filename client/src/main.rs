use sycamore::{prelude::*, reactive::ReadSignal, suspense::Suspense};
use sycamore_router::{HistoryIntegration, Route, Router};
use web_sys::window;

use crate::components::{index::Index, choose::Fighters};

mod api;
mod components;

#[derive(Route, Clone, Copy)]
enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/choose")]
    Choose,
    #[not_found]
    NotFound,
}

#[component]
pub fn App<G: Html>() -> View<G> {
    window().unwrap().document().unwrap().set_title("ebobo.dev");

    view! {
        Router(
            integration=HistoryIntegration::new(),
            view=|route: ReadSignal<AppRoutes>| {
                view! {
                    div(class="app") {
                        (match route.get() {
                            AppRoutes::Index => view! {
                                Suspense(fallback=view! { "Loading..." }) {
                                    Index { }
                                }
                            },
                            AppRoutes::Choose => view! {
                                Suspense(fallback=view! { "Loading..." }) {
                                    Fighters { }
                                }
                            },
                            AppRoutes::NotFound => view! {
                                "404 Not Found"
                            },
                        })
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
