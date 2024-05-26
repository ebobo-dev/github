use sycamore::{prelude::*, reactive::ReadSignal, suspense::Suspense};
use sycamore_router::{HistoryIntegration, Route, Router};
use web_sys::window;

use components::{choose::Fighters, index::Index};
use routing::*;

mod api;
mod components;
mod pages;
mod routing;

#[component]
pub fn App<G: Html>() -> View<G> {
    window().unwrap().document().unwrap().set_title("ebobo.dev");

    view! {
        Router(
            integration=HistoryIntegration::new(),
            view=|route: ReadSignal<AppRoutes>| view! {
                div(class="app") {
                    components::index::Index()
                    div(class="container") {
                        Switch(route=route)
                    }
                }
            }
        )

        // Router(
        //     integration=HistoryIntegration::new(),
        //     view=|route: ReadSignal<AppRoutes>| {
        //         view! {
        //             div(class="app") {
        //                 (match route.get() {
        //                     AppRoutes::Index => view! {
        //                         Suspense(fallback=view! { "loading..." }) {
        //                             Index { }
        //                         }
        //                     },
        //                     AppRoutes::Choose => view! {
        //                         Suspense(fallback=view! { "loading available fighters..." }) {
        //                             Fighters { }
        //                         }
        //                     },
        //                     AppRoutes::NotFound => view! {
        //                         "404 Not Found"
        //                     },
        //                 })
        //             }
        //         }
        //     }
        // )

    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(App);
}
