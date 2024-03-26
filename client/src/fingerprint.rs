use serde::Deserialize;
use wasm_fingerprint::make_fingerprint;

#[derive(Deserialize)] 
struct Fingerprint {
    print: String,
}

pub fn fingerprint() -> String {
    let fingerprint: Fingerprint =
        serde_json::from_str(&make_fingerprint().expect("Fingerprint not available"))
            .expect("Failed to deserialize fingerprint");
    fingerprint.print
}
