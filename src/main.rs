use serde::Deserialize;
use wasm_fingerprint::*;
use web_sys::*;
use yew::prelude::*;

#[derive(Deserialize)]
struct Fingerprint {
    print: String,
}

#[function_component(WhoAmI)]
fn fingerprint() -> Html {
    let fingerprint = make_fingerprint().unwrap();
    let fingerprint: Fingerprint = serde_json::from_str(&fingerprint).unwrap();

    html! {
        <div>

        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let window = window().unwrap();
    let document = window.document().unwrap();

    const NAME: &str = "ebobo.dev";

    document.set_title(NAME);

    let email = format!("generated@{}", NAME);

    html! {
        <div>
            <h1>{ NAME } </h1>
            <WhoAmI />
            <footer>
                <a href={ format!("mailto:{}", email) }>{ email }</a>
            </footer>
            <!-- 
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
