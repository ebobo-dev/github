use sycamore::{futures::spawn_local, prelude::*};

use crate::{api::choose, AvailableFighters};

#[component(inline_props)]
pub async fn Fight<G: Html>() -> View<G> {
    let fighters = create_signal(use_context::<AvailableFighters>());
    let selected: Signal<Option<String>> = create_signal(None);

    create_effect(move || {
        if let Some(fighter) = selected.get() {
            spawn_local(async move {
                match choose(fighter.to_string()).await {
                    Ok(_) => (),
                    Err(err) => log::error!("error: {:?}", err), // TODO: handle error
                }
            });
        }
    });

    view! {
        div {
            p { "choose your fighter:" }
            ul {
                Indexed(
                    iterable = *fighters.get().0.iter(),
                    view = move |f| view! {
                        li {
                            button(on:click = move |_| selected.set(Some(f))) { (f) }
                        }
                    }
                )
            }
        }
    }
}
