use sycamore::prelude::*;
use sycamore_router::navigate;
use crate::components::choose::Fighters;

use crate::api::*;

#[component]
pub async fn Index<G: Html>() -> View<G> {
    let index = get().await.expect("failed to get greeting");
    match index.fighter {
        Some(_) => {
            navigate("arena");
            view! { p { (index.greet) } }
        },
        None => {
            view! { 
                p { (index.greet) }
                Fighters()
            }
        }
    }
}
