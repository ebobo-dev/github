use crate::AppState;
use ebobo_shared::*;
use libsql::{params, Row};
use rocket::http::ext::IntoCollection;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use std::sync::Arc;

#[post("/authenticate", data = "<request>")]
pub async fn authenticate(
    request: Json<Auth>,
    state: &State<Arc<AppState>>,
) -> Result<String, BadRequest<String>> {
    let res = state
        .db
        .lock()
        .await
        .query(
            "SELECT * FROM devices WHERE fingerprint = ?1",
            params!(request.fingerprint.to_owned()),
        )
        .await
        .unwrap();


   
    Ok("Hi, world!".to_owned())
}
