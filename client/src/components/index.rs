use sycamore::prelude::*;

use crate::api::get;

#[component(inline_props)]
pub async fn Index<G: Html>() -> View<G> {
    let greet = get().await;
    let greet = match greet {
        Ok(g) => g,
        Err(e) => e.to_string(),
    };

    view! {
        p {
            (greet)
        }
    }
}
