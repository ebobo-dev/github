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
            <h2>{ format!("Fingerprint: {}", fingerprint.print) }</h2>
            <button onclick={Callback::from(|_| {
                let window = window().unwrap();
                let document = window.document().unwrap();
                document.set_title("Saved!");
            })}>{ "Save" }</button>
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
            <h1>{ "Hello, Микола!" } </h1>
            <WhoAmI />
            <footer>
                <a href={ format!("mailto:{}", email) }>{ email }</a>
            </footer>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render(); // TODO: check csr dependency.
}
