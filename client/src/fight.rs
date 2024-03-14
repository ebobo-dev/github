use ebobo_shared::Fighter;
use sycamore::prelude::*;

#[component(inline_props)]
pub async fn Fight<G: Html>(url: String) -> View<G> {
    let fighters = create_signal(vec!["🐱", "🐵", "🐶", "🐷"]);
    let state: Signal<Option<&str>> = create_signal(None);

    create_effect(move || {
        if let Some(fighter) = state.get() {
            let url = url.clone();
            let fighter = fighter.to_string();
            let _ = async move {
                match post(&url, &fighter).await {
                    Ok(_) => (),
                    Err(err) => log::error!("error: {:?}", err),
                }
            };
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
                fingerprint: "fingerprint".to_string(),
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
