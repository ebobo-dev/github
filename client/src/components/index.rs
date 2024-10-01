use sycamore::prelude::*;
use sycamore_router::navigate;
use web_sys::wasm_bindgen::UnwrapThrowExt;

use crate::components::choose::Fighters;

#[component]
pub async fn Index() -> View {
    let index = crate::api::index().await.unwrap_throw();

    if let Some(_) = index.fighter {
        navigate("arena");
    }

    view! {
        p { (index.greet.clone()) }
        Fighters()
    }
}
