use sycamore::prelude::*;
use sycamore_router::navigate;

use crate::api::*;

#[component]
pub async fn Index<G: Html>() -> View<G> {
    let index = get().await.expect("failed to get greeting");
    //if !index.fighter { navigate("choose") }
    view!()
}
