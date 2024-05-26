use sycamore::prelude::*;
use sycamore_router::navigate;

use crate::api::*;

#[component]
pub async fn Index<G: Html>() -> View<G> {
    let index = get().await.expect("failed to get greeting");
    match index.fighter {
        Some(f) => view! { (f) },
        None => {
            navigate("choose");
            view! { }
        }
    }
}
