use wasm_fingerprint::make_fingerprint;
use serde::Deserialize;

pub fn fingerprint() -> String {
    let fingerprint: Fingerprint =
        serde_json::from_str(&make_fingerprint().expect("fingerprint not available"))
            .expect("failed to deserialize fingerprint");
    fingerprint.print
}

#[derive(Deserialize)]
struct Fingerprint {
    print: String,
}