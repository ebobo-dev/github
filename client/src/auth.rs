use ebobo_shared::Auth;
use reqwasm::http::Request;
use serde::Deserialize;
use sycamore::prelude::*;
use wasm_fingerprint::make_fingerprint;
use web_sys::window;

#[component]
pub async fn Auth<G: Html>() -> View<G> {
    let fingerprint: Fingerprint = serde_json::from_str(&make_fingerprint().unwrap()).unwrap();
    let addr = window().unwrap().location().host().unwrap();
    let greet = post(&fingerprint.print, &addr).await.unwrap();

    view! {
        p {
            (greet)
        }
    }
}

async fn post(fingerprint: &str, host: &str) -> Result<String, reqwasm::Error> {
    let url = std::env::var("EBOBO_AUTH_URL").unwrap_or_else(|_| "http://localhost:8000/authenticate".to_string());
    Ok(Request::post(&url)
        .body(
            serde_json::to_string(&Auth {
                fingerprint: fingerprint.to_string(),
                addr: host.to_string(),
            })
            .unwrap(),
        )
        .send()
        .await?.text().await?)
}

#[derive(Deserialize)]
struct Fingerprint {
    print: String,
}

