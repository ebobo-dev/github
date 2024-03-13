mod auth;
mod fight;

use sycamore::{prelude::*, suspense::Suspense};
use web_sys::window;
use auth::Auth;
use fight::Fight;

#[component]
fn App<G: Html>() -> View<G> {
    const NAME: &str = "ebobo.dev";
    let email = format!("generated@{}", NAME);
    let link = format!("mailto:{}", email);
    window().unwrap().document().unwrap().set_title(NAME);

    

    view! {
        div {
            h1 { (NAME) }

            Suspense(fallback=view! { "Loading..." }) {
                Auth {}
            }

            Fight {}

            footer {
                a(href=link) {
                    (email)
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
