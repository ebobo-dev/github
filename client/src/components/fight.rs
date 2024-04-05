use sycamore::{futures::spawn_local, prelude::*, web::html::a};

use crate::{api::choose, AvailableFighters};

#[component(inline_props)]
pub async fn Fight<G: Html>() -> View<G> {
    let available = use_context::<Signal<AvailableFighters>>();
    let available = available.get().as_ref().get();
    let fighters = create_signal(fighters);
    let selected: Signal<Option<&str>> = create_signal(None);

    create_effect(move || {
        if let Some(fighter) = selected.get() {
            spawn_local(async move {
                match choose(fighter.to_string()).await {
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
                            button(on:click = move |_| selected.set(Some(f))) { (f) }
                        }
                    }
                )
            }
        }
    }
}
