use sycamore::prelude::*;

use crate::api::*;

#[component(inline_props)]
pub async fn Index<G: Html>() -> View<G> {
    let index = get().await.expect("failed to get greeting");

    match index.fighter {
        false => view! {
            p {
                (index.greet)
            }
        },
        true => view! {
            p {
                (index.greet)
            }
        },
    }
}
