use sycamore::prelude::*;

#[component(inline_props)]
pub async fn Index<G: Html>() -> View<G> {
    //let fighters = create_signal(get().await.expect("Index call failed"));

    view! {
        p {
            "Fighters:"
        }
    }
}