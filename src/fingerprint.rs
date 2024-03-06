use std::fmt::*;
use serde::Deserialize;
use wasm_fingerprint::*;
use yew::prelude::*;

#[derive(Deserialize)]
struct Fingerprint {
    print: String,
    ms: f32
}

impl Display for Fingerprint {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "fingerprint: {}, time: {}", self.print, self.ms)
    }
}

#[function_component(WhoAmI)]
pub fn fingerprint() -> Html {
    let fingerprint = make_fingerprint().unwrap();
    let fingerprint: Fingerprint = serde_json::from_str(&fingerprint).unwrap();
    
    html! {
        <div>{ fingerprint }</div>
    }
}