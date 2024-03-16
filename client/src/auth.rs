use crate::*;
use ebobo_shared::Fighter;
use reqwasm::http::Request;
use sycamore::prelude::*;

#[component(inline_props)]
pub async fn Auth<G: Html>() -> View<G> {
    let fighter = post(&url(), &fingerprint()).await.unwrap();

    let greet = match fighter.fighter {
        Some(f) => format!("Welcome back, {}!", f),
        None => format!("Welcome, {}!", fighter.fingerprint),
    };

    view! {
        p {
            (greet)
        }
    }
}

async fn post(url: &str, fingerprint: &str) -> Result<Fighter, reqwasm::Error> {
    Ok(Request::post(format!("{}/authenticate", url).as_str())
        .header(ebobo_shared::AUTH_HEADER, fingerprint)
        .send()
        .await?
        .json()
        .await?)
}
