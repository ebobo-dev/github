use sycamore::prelude::*;

use crate::api::*;
use crate::components::choose::Fighters;

#[component(inline_props)]
pub async fn Index<G: Html>() -> View<G> {
    let index = get().await.expect("Failed to get page index");

    match index.fighter {
        false => view! {
            p {
                (index.greet)
            }

            Fighters(fighters = index.fighters)
        },
        true => view! {
            p {
                (index.greet)
            }
        },
    }
}
