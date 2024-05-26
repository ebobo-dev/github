use sycamore::{prelude::*, reactive::ReadSignal, suspense::Suspense};
use sycamore_router::{HistoryIntegration, Route, Router};

#[component(inline_props)]
pub async fn Index<G: Html>() -> View<G> {
    view!()
}