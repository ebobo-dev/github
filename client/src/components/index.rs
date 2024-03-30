use sycamore::prelude::*;

use crate::api::get;

#[component(inline_props)]
pub async fn Index<G: Html>() -> View<G> {
    let fighters = get().await.expect("Index call failed");
    let state = create_signal(fighters);

    view! {
        p {
            "Fighters:"
        }
    }
}