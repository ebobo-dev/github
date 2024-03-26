use reqwasm::http::Request;
use sycamore::prelude::*;

use ebobo_shared::Fighter;

use crate::*;
use crate::fingerprint::fingerprint;

#[component(inline_props)]
pub async fn Auth<G: Html>() -> View<G> {
    let fighter = post(&url(), &fingerprint()).await.expect("Authentication failed");

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
