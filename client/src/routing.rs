use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

use crate::components;

#[component(inline_props)]
pub async fn Root<G: Html>() -> View<G> {
    view! {
        Router(
            integration=HistoryIntegration::new(),
            view=|route: ReadSignal<AppRoutes>| view! {
                Switch(route=route)
            }
        )
    }
}

#[component(inline_props)]
async fn Switch<G: Html>(route: ReadSignal<AppRoutes>) -> View<G> {
    view! {(match route.get() {
            AppRoutes::Index => view! { components::index::Index() },
            AppRoutes::Arena => view! { components::arena::Dashboard() },
            AppRoutes::NotFound => view! { "lost?"}
        })
    }
}

#[derive(Route, Clone, Copy, Debug)]
enum AppRoutes {
    #[to("")]
    Index,
    #[to("/arena")]
    Arena,
    #[not_found]
    NotFound,
}
