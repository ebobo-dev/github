use sycamore::prelude::*;
use web_sys::window;

#[component]
pub fn Footer<G: Html>() -> View<G> {
    const NAME: &str = "ebobo.dev";
    let email = format!("generated@{}", NAME);
    let link = format!("mailto:{}", email);
    window().unwrap().document().unwrap().set_title(NAME);

    view! {
        footer { a(href=link) { (email) } }
        a(href="https://github.com/dotnicht/ebobo") { 
            img(src="./img/github-mark.png", alt="GitHub") 
        }
    }
}
