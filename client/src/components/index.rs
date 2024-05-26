use sycamore::prelude::*;
use crate::components::choose::Fighters;

use crate::api::*;

#[component]
pub async fn Index<G: Html>() -> View<G> {
    let index = get().await.expect("failed to get greeting");
    match index.fighter {
        Some(_) => view! { },
        None => {
            view! { 
                Fighters()
            }
        }
    }
}
