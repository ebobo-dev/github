use sycamore::prelude::*;

#[component]
pub async fn Fight<G: Html>() -> View<G> {
    let fighters = create_signal(vec!["🐱", "🐵", "🐶", "🐷"]);
    let fighter: Signal<Option<&str>> = create_signal(None);

    let save = {
        let fighter = fighter.clone();
        move |_| {
            let f = fighter.get().unwrap();
            let _ = post(f);
        }
    };

    view! {
        div {
            p { "choose your fighter:" }
            ul {
                Indexed(
                    iterable = *fighters,
                    view = move |f| view! {
                        li {
                            button(on:click=move |_| fighter.set(Some(f))) { (f) }
                        }
                    }
                )
            }
            p {
                "you chose: "
                (fighter.get().unwrap_or("nothing"))
                button(on:click=save) { "save" }
            }
        }
    }
}

async fn post(fighter: &str) -> Result<(), reqwasm::Error> {
    match reqwasm::http::Request::post("https://ebobo.shuttleapp.rs/choose")
            .body(
                serde_json::to_string(&ebobo_shared::Fighter {
                    fingerprint: "fingerprint".to_string(),
                    fighter: Some(fighter.to_string()),
                })
                .unwrap(),
            )
            .send()
            .await {
        Ok(_) => Ok(()),
        Err(err) => return Err(err),
    }
}

