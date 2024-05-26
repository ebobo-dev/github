use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

use crate::pages;

#[derive(Route, Clone, Copy, Debug)]
pub enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/choose")]
    Choose,
    #[not_found]
    NotFound,
}

#[component(inline_props)]
pub async fn Switch<G: Html>(route: ReadSignal<AppRoutes>) -> View<G> {
    view! {(match route.get() {
            AppRoutes::Index => view! { pages::index::Index() },
            AppRoutes::Choose => view! { pages::choose::Choose() },
            AppRoutes::NotFound => view! { "404 Page Not Found"}
        })
    }
}
