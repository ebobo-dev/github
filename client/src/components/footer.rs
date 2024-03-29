use sycamore::prelude::*;
use web_sys::window;

#[component]
pub fn Footer<G: Html>() -> View<G> {
    const NAME: &str = "ebobo.dev";

    if let Some(w) = window() {
        if let Some(d) = w.document() {
            d.set_title(NAME);
        }
    }

    let email = format!("generated@{}", NAME);
    let link = format!("mailto:{}", email);

    view! {
        footer {
            a(href=link) { (email) }
            a(href="https://github.com/dotnicht/ebobo") { img(src="./img/github-mark.png", alt="GitHub", width="24", height="24") }
        }
    }
}
