use sycamore::prelude::*;
use sycamore_router::navigate;

use crate::api::*;
use crate::components::choose::Fighters;

#[component]
pub async fn Index<G: Html>() -> View<G> {
    let index = get().await.expect("failed to get greeting");

    if let Some(_) = index.fighter {
        navigate("arena");
    }

    view! {
        p { (index.greet) }
        Fighters()
    }
}
