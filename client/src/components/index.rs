use sycamore::prelude::*;

use crate::api::get;

#[component(inline_props)]
pub async fn Index<G: Html>() -> View<G> {
    let greet = match get().await {
        Ok(g) => g.greet,
        Err(e) => e.to_string(),
    };

    view! {
        p {
            (greet)
        }
    }
}
