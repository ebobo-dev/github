use sycamore::futures::spawn_local;
use sycamore::prelude::*;

use ebobo_shared::Fighter;
use sycamore_router::navigate;
use web_sys::wasm_bindgen::UnwrapThrowExt;

use crate::api::*;

#[component]
pub async fn Fighters<G: Html>() -> View<G> {
    let fighters = available().await.expect("failed to get available fighters");
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
    let name = fighter.name();
    let selected: Signal<Option<Fighter>> = create_signal(None);

    create_effect(move || {
        if let Some(fighter) = selected.get_clone() {
            spawn_local(async move {
                choose(fighter.name()).await.unwrap_throw();
            });
        }
    });

    view! {
        li {
            button(on:click = move |_| {
                selected.set(Some(fighter.clone()));
                navigate("arena")
            }) { (name) }
        }
    }
}
