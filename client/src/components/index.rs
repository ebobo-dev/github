use sycamore::prelude::*;

use crate::api::get;
use crate::*;

#[component(inline_props)]
pub async fn Index<G: Html>() -> View<G> {
    let available = create_signal(AvailableFighters(vec![]));
    provide_context(available);

    let greet = match get().await {
        Ok(g) => {
            available.set(AvailableFighters(g.fighters.clone()));
            g.greet
        },
        Err(e) => e.to_string(),
    };

    view! {
        p {
            (greet)
        }
    }
}
