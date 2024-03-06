use serde::Deserialize;
use wasm_fingerprint::*;
use yew::prelude::*;

#[derive(Deserialize)]
struct Fingerprint {
    print: String,
}

#[function_component(WhoAmI)]
pub fn fingerprint() -> Html {
    let fingerprint = make_fingerprint().unwrap();
    let fingerprint: Fingerprint = serde_json::from_str(&fingerprint).unwrap();
    
    html! {
        <div>{ fingerprint.print }</div>
    }
}