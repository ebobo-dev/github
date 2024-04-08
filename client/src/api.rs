use reqwasm::http::Request;
use serde::Deserialize;
use wasm_fingerprint::make_fingerprint;

use ebobo_shared::*;

#[derive(Deserialize)]
struct Fingerprint {
    print: String,
}

fn fingerprint() -> String {
    let fingerprint: Fingerprint =
        serde_json::from_str(&make_fingerprint().expect("Fingerprint not available"))
            .expect("Failed to deserialize fingerprint");
    fingerprint.print
}

fn url() -> String {
    option_env!("EBOBO_API_URL")
        .unwrap_or("https://ebobo.shuttleapp.rs")
        .to_owned()
}

pub async fn get() -> Result<Index, reqwasm::Error> {
    Ok(Request::get(&url())
        .header(ebobo_shared::AUTH_HEADER, &fingerprint())
        .send()
        .await?
        .json()
        .await?)
}

pub async fn choose(fighter: String) -> Result<Choice, reqwasm::Error> {
    Ok(Request::post(format!("{}/choose", url()).as_str())
        .header(ebobo_shared::AUTH_HEADER, &fingerprint())
        .body(serde_json::to_string(&Choice::new(&fighter)).expect("Failed to serialize choice"))
        .send()
        .await
        .map_err(|e| reqwasm::Error::from(e))?
        .json()
        .await
        .map_err(|e| reqwasm::Error::from(e))?)
}
