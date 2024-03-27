use reqwasm::http::Request;
use sycamore::prelude::*;

use ebobo_shared::Fighter;

use crate::fingerprint::fingerprint;
use crate::*;

#[component(inline_props)]
pub async fn Index<G: Html>() -> View<G> {
    let fighters = create_signal(get(&fingerprint()).await.expect("Authentication failed"));

    view! {
        p {
            ul {
                Indexed(
                    iterable = *fighters,
                    view = move |f| view! {
                        li {
                            (format!("{}: {}", f.fingerprint, f.fighter.as_deref().unwrap_or("Anonymous")))
                        }
                    }
                )
            }
        }
    }
}

async fn get(fingerprint: &str) -> Result<Vec<Fighter>, reqwasm::Error> {
    Ok(Request::post(&url())
        .header(ebobo_shared::AUTH_HEADER, fingerprint)
        .send()
        .await?
        .json()
        .await?)
}
