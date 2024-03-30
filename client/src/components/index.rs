use sycamore::prelude::*;

use crate::api::get;

#[component(inline_props)]
pub async fn Index<G: Html>() -> View<G> {
    let fighters = get().await;
    let greet = match fighters {
        Ok(fighters) => fighters.len().to_string(),
        Err(e) => e.to_string(),
    };

    view! {
        p {
            (greet)
        }
    }
}
