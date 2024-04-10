use sycamore::prelude::*;

use crate::api::*;
use crate::components::fighters::Fighters;

#[component(inline_props)]
pub async fn Index<G: Html>() -> View<G> {
    let index = get().await.unwrap();
    //let select = !index.fighter;

    view! {
            p {
                (index.greet)
            }
            
            Fighters(fighters = index.fighters)
    }
}
