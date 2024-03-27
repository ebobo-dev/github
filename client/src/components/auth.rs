use sycamore::prelude::*;

use crate::api::auth;

#[component(inline_props)]
pub async fn Auth<G: Html>() -> View<G> {
    auth().await.expect("Authentication failed");

    view! {
        p { }
    }
}
