use reqwasm::http::Request;
use serde::Deserialize;
use wasm_fingerprint::make_fingerprint;

use ebobo_shared::Fighter;

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

pub async fn get() -> Result<Vec<Fighter>, reqwasm::Error> {
    Ok(Request::get(&url())
        .header(ebobo_shared::AUTH_HEADER, &fingerprint())
        .send()
        .await?
        .json()
        .await?)
}

pub async fn auth() -> Result<(), reqwasm::Error> {
    match Request::post(format!("{}/authenticate", url()).as_str())
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn choose(fighter: &str) -> Result<(), reqwasm::Error> {
    match Request::post(format!("{}/choose", url()).as_str())
        .body(
            serde_json::to_string(&Fighter {
                fingerprint: fingerprint(),
                fighter: Some(fighter.to_string()),
            })
            .unwrap(),
        )
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
