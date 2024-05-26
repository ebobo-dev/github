use sycamore::prelude::*;

#[component]
pub async fn Dashboard<G: Html>() -> View<G> {
    view!("welcome to the arena")
}