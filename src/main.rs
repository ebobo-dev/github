use serde::Deserialize;
use wasm_fingerprint::*;
use yew::prelude::*;

#[derive(Deserialize)]
struct Fingerprint {
    print: String,
}

#[function_component(App)]
fn app() -> Html {
    let fingerprint = make_fingerprint().unwrap();
    let fingerprint: Fingerprint = serde_json::from_str(&fingerprint).unwrap();

    if fingerprint.print == "10E14AEC" {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        document.set_title("привет еблан!");
    }

    html! {
        <div>
            <h1>{ "ebobo.dev" }</h1>
            <footer>
                <a href="mailto:generated@ebobo.dev">{"generated@ebobo.dev"}</a>
            </footer>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render(); // TODO: check csr dependency.
}
