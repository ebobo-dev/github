use sycamore::prelude::*;

use crate::api::get;

#[component(inline_props)]
pub async fn Index<G: Html>() -> View<G> {
    let greet = get().await;
    let greet = match greet {
        Ok(fighters) => fighters,
        Err(e) => e.to_string(),
    };

    view! {
        p {
            (greet)
        }
    }
}
