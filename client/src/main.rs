mod auth;
mod fight;
mod footer;

use auth::Auth;
use fight::Fight;
use footer::Footer;
use serde::Deserialize;
use sycamore::{prelude::*, suspense::Suspense};
use wasm_fingerprint::make_fingerprint;

pub fn url() -> String {
    option_env!("EBOBO_API_URL").unwrap_or("http://localhost:8000").to_owned()
}

pub fn fingerprint() -> String {
    let fingerprint: Fingerprint = serde_json::from_str(&make_fingerprint().unwrap()).unwrap();
    fingerprint.print
}

#[derive(Deserialize)]
pub struct Fingerprint {
    print: String,
}

#[component]
fn App<G: Html>() -> View<G> {
    view! {
        div {
            Suspense(fallback=view! { "Loading..." }) {
                Auth { }
            }

            Fight { }
            
            Footer { }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(App);
}
