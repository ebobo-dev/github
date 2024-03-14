use ebobo_shared::Fighter;
use sycamore::{futures::spawn_local, prelude::*};

use crate::{fingerprint, url};

#[component(inline_props)]
pub async fn Fight<G: Html>() -> View<G> {
    let fighters = create_signal(vec!["🐱", "🐵", "🐶", "🐷"]);
    let state: Signal<Option<&str>> = create_signal(None);

    create_effect(move || {
        if let Some(fighter) = state.get() {
            let url = url();
            let fighter = fighter.to_string();
            spawn_local(async move {
                match post(&url, &fighter).await {
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

async fn post(url: &str, fighter: &str) -> Result<(), reqwasm::Error> {
    match reqwasm::http::Request::post(format!("{}/choose", url).as_str())
        .body(
            serde_json::to_string(&Fighter {
                fingerprint: fingerprint(),
                fighter: Some(fighter.to_string()),
            })
            .unwrap(),
        )
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
