use std::vec;

use sycamore::prelude::*;
use sycamore::futures::spawn_local;

use crate::api::*;

#[component(inline_props)]
pub async fn Index<G: Html>() -> View<G> {
    let available = create_signal(vec![]);

    let greet = match get().await {
        Ok(g) => {
            available.set(g.fighters.clone());
            g.greet
        },
        Err(e) => e.to_string(),
    };

    let selected: Signal<Option<&str>> = create_signal(None);

    create_effect(move || {
        if let Some(fighter) = selected.get() {
            spawn_local(async move {
                match choose(fighter.to_owned()).await {
                    Ok(_) => (),
                    Err(err) => log::error!("error: {:?}", err),
                }
            });
        }
    });

    view! {
        p {
            (greet)
        }
        div {
            p { "choose your fighter:" }
            ul {
                Indexed(
                    iterable = *available,
                    view = move |f| view! {
                        li {
                            button(on:click = move |_| selected.set(None)) { (f) }
                        }
                    }
                )
            }
        }
    }
}
