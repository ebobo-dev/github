use ebobo_shared::Auth;
use reqwasm::http::Request;
use serde::Deserialize;
use sycamore::prelude::*;
use wasm_fingerprint::make_fingerprint;
use web_sys::RequestMode;

#[component]
pub async fn Auth<G: Html>() -> View<G> {
    let fingerprint: Fingerprint = serde_json::from_str(&make_fingerprint().unwrap()).unwrap();
    
    let addr = Request::get("https://api.ipify.org")
        .mode(RequestMode::Cors)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let greet = post(&fingerprint.print, &addr).await.unwrap();

    view! {
        p {
            (greet)
        }
    }
}

async fn post(fingerprint: &str, host: &str) -> Result<String, reqwasm::Error> {
    Ok(Request::post("https://ebobo.shuttleapp.rs/authenticate")
        .body(
            serde_json::to_string(&Auth {
                fingerprint: fingerprint.to_string(),
                addr: host.to_string(),
            })
            .unwrap(),
        )
        .send()
        .await?
        .text()
        .await?)
}

#[derive(Deserialize)]
struct Fingerprint {
    print: String,
}
