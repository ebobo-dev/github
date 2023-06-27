use serde::Deserialize;
use wasm_fingerprint::*;
use web_sys::*;
use yew::prelude::*;

#[derive(Deserialize)]
struct Fingerprint {
    print: String,
}

#[function_component(App)]
fn app() -> Html {
    let window = window().unwrap();
    let document = window.document().unwrap();
    document.set_title("ebobo.dev");

    let fingerprint = make_fingerprint().unwrap();
    let fingerprint: Fingerprint = serde_json::from_str(&fingerprint).unwrap();

    html! {
        <div>
            <h1>{ fingerprint.print }</h1>
            <footer>
                <a href="mailto:generated@ebobo.dev">{"generated@ebobo.dev"}</a>
            </footer>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render(); // TODO: check csr dependency.
}
