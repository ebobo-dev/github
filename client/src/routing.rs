use sycamore::prelude::*;
use sycamore_router::Route;

use crate::components;

#[derive(Route, Clone, Copy, Debug)]
pub enum AppRoutes {
    #[to("/")]
    Index,
    #[not_found]
    NotFound,
}

#[component(inline_props)]
pub async fn Switch<G: Html>(route: ReadSignal<AppRoutes>) -> View<G> {
    view! {(match route.get() {
            AppRoutes::Index => view! { components::index::Index() },
            AppRoutes::NotFound => view! { "lost?"}
        })
    }
}
