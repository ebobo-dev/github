use sycamore::{futures::spawn_local, prelude::*};

use crate::api::choose;

#[component(inline_props)]
pub async fn Fight<G: Html>() -> View<G> {
    let fighters = create_signal(vec!["🐱", "🐵", "🐶", "🐷"]);
    let state: Signal<Option<&str>> = create_signal(None);

    create_effect(move || {
        if let Some(fighter) = state.get() {
            let fighter = fighter.to_string();
            spawn_local(async move {
                match choose(&fighter).await {
                    Ok(_) => (),
                    Err(err) => log::error!("error: {:?}", err),
                }
            });
        }
    });

    view! {
        div {
            p { "choose your fighter:" }
            ul {
                Indexed(
                    iterable = *fighters,
                    view = move |f| view! {
                        li {
                            button(on:click = move |_| state.set(Some(f))) { (f) }
                        }
                    }
                )
            }
        }
    }
}
