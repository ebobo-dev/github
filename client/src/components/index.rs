use std::vec;

use sycamore::futures::spawn_local;
use sycamore::prelude::*;

use crate::api::*;

#[component(inline_props)]
pub async fn Index<G: Html>() -> View<G> {
    let available: Signal<Vec<String>> = create_signal(vec![]);

    let greet = match get().await {
        Ok(g) => {
            available.set(g.fighters);
            g.greet
        }
        Err(e) => e.to_string(),
    };

    view! {
        p {
            (greet)
        }

        Fight(fighters = available) { }
    }
}

#[component(inline_props)]
pub async fn Fight<G: Html>(fighters: Signal<Vec<String>>) -> View<G> {
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

    view!( div {
        p { "choose your fighter:" }
        ul {
            Indexed(
                iterable = *fighters,
                view = move |f| view! {
                    li {
                        button(on:click = move |_| selected.set(None)) { (f) }
                    }
                }
            )
        }
    })
}
