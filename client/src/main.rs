mod auth;
mod fight;
mod fingerprint;
mod footer;

use auth::Auth;
use fight::Fight;
use footer::Footer;
use sycamore::{prelude::*, suspense::Suspense};

pub fn url() -> String {
    option_env!("EBOBO_API_URL")
        .unwrap_or("https://ebobo.shuttleapp.rs")
        .to_owned()
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
