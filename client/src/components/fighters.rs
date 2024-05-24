use sycamore::futures::spawn_local;
use sycamore::prelude::*;

use ebobo_shared::Fighter;

use crate::api::*;

#[component(inline_props)]
pub async fn Fighters<G: Html>(fighters: Vec<Fighter>) -> View<G> {
    let size = fighters.len();
    let available = create_signal(fighters);

    if size == 0 {
        view!(p { "no fighters available" })
    } else {
        view!( div {
            p { "choose your fighter:" }
            ul {
                Indexed(
                    iterable = *available,
                    view = move |f| view! {
                        SelectFighter(fighter = f.clone())
                    }
                )
            }
        })
    }
}

#[component(inline_props)]
pub async fn SelectFighter<G: Html>(fighter: Fighter) -> View<G> {
    let name = fighter.clone().name();
    let selected: Signal<Option<Fighter>> = create_signal(None);

    create_effect(move || {
        if let Some(fighter) = selected.get_clone() {
            spawn_local(async move {
                match choose(fighter.name()).await {
                    Ok(_) => (),
                    Err(err) => log::error!("error: {:?}", err),
                }
            });
        }
    });

    view! {
        li {
            button(on:click = move |_| selected.set(Some(fighter.clone()))) { (name) }
        }
    }
}
