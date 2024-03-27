use sycamore::prelude::*;

use crate::api::get;

#[component(inline_props)]
pub async fn Index<G: Html>() -> View<G> {
    let fighters = create_signal(get().await.expect("Index call failed"));

    view! {
        p {
            ul {
                Indexed(
                    iterable = *fighters,
                    view = move |f| view! {
                        li {
                            (format!("{}: {}", f.fingerprint, f.fighter.as_deref().unwrap_or("Anonymous")))
                        }
                    }
                )
            }
        }
    }
}