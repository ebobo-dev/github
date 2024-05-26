use sycamore::prelude::*;

use crate::api::*;

#[component(inline_props)]
pub async fn Index<G: Html>() -> View<G> {
    let index = get().await.expect("failed to get greeting");

    view! {
        p {
            (index.greet)
        }
    }
}
