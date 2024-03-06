use std::fmt::*;
use serde::Deserialize;
use wasm_fingerprint::*;
use yew::prelude::*;

#[derive(Deserialize, Debug)]
struct Fingerprint {

    print: String,
}

impl Display for Fingerprint {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "fingerprint: {}", self.print)
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