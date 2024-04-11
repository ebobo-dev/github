use sycamore::prelude::*;

use crate::api::*;
use crate::components::fighters::Fighters;

#[component(inline_props)]
pub async fn Index<G: Html>() -> View<G> {
    let index = get().await.expect("Failed to get page index");
    let select = !index.fighter;

    match select {
        true => view! {
            p {
                (index.greet)
            }

            Fighters(fighters = index.fighters)
        },
        false => view! {
            p {
                (index.greet)
            }
        },
    }
}
