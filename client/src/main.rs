use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use sycamore::{prelude::*, suspense::Suspense};
use wasm_fingerprint::make_fingerprint;

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    value: String,
    address: SocketAddr,
}

#[derive(Deserialize)]
struct Fingerprint {
    print: String,
}

const API: &str = "https://ebobo.shuttleapp.rs";

async fn post(fingerprint: &str) -> Result<String, reqwasm::Error> {
    let resp = Request::post(API)
        .body(fingerprint)
        .send()
        .await?;

    let body = resp.text().await?;
    Ok(body)
}

#[component]
async fn Auth<G: Html>() -> View<G> {
    let fingerprint = make_fingerprint().unwrap();
    let fingerprint: Fingerprint = serde_json::from_str(&fingerprint).unwrap();

    let greet = post(&fingerprint.print).await.unwrap_or_default();

    view! {
        p {
            (greet)
        }
    }
}

#[component]
fn App<G: Html>() -> View<G> {
    view! {
        div {
            p { "ebobo.dev" }

            Suspense(fallback=view! { "Loading..." }) {
                Auth {}
            }

            footer {
                a(href="mailto:developed@ebobo.dev") {
                    "developed@ebobo.dev"
                }
            }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(App);
}
