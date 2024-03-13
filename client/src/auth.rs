use ebobo_shared::{Auth, Fighter};
use reqwasm::http::Request;
use serde::Deserialize;

use sycamore::prelude::*;
use wasm_fingerprint::make_fingerprint;
use web_sys::RequestMode;

#[component(inline_props)]
pub async fn Auth<G: Html>(url: String) -> View<G> {
    let fingerprint: Fingerprint = serde_json::from_str(&make_fingerprint().unwrap()).unwrap();

    let addr = Request::get("https://api.ipify.org")
        .mode(RequestMode::Cors)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let fighter = post(url.as_str(), &fingerprint.print, &addr).await.unwrap();

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

async fn post(url: &str, fingerprint: &str, host: &str) -> Result<Fighter, reqwasm::Error> {
    Ok(Request::post(format!( "{}/authenticate", url).as_str())
        .body(
            serde_json::to_string(&Auth {
                fingerprint: fingerprint.to_string(),
                addr: host.to_string(),
            })
            .unwrap(),
        )
        .send()
        .await?
        .json()
        .await?)
}

#[derive(Deserialize)]
struct Fingerprint {
    print: String,
}
