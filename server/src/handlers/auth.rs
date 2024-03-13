use crate::AppState;
use ebobo_shared::*;
use libsql::de::from_row;
use libsql::params;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use std::sync::Arc;

#[post("/authenticate", data = "<request>")]
pub async fn authenticate(
    request: Json<Auth>,
    state: &State<Arc<AppState>>,
) -> Result<Json<Fighter>, BadRequest<String>> {
    let mut res = state
        .db
        .lock()
        .await
        .query(
            "SELECT * FROM devices WHERE fingerprint = ?1",
            params!(request.fingerprint.to_owned()),
        )
        .await
        .map_err(|e| BadRequest(e.to_string()))?;

    match res.next() {
        Ok(None) => {
            state
                .db
                .lock()
                .await
                .execute(
                    "INSERT INTO devices (fingerprint) VALUES (?1)",
                    params!(request.fingerprint.to_owned()),
                )
                .await
                .map_err(|e| BadRequest(e.to_string()))?;
            Ok(Json(Fighter {
                fingerprint: request.fingerprint.to_owned(),
                fighter: None,
            }))
        }
        Ok(Some(d)) => Ok(Json(from_row::<Fighter>(&d).unwrap())),
        Err(e) => Err(BadRequest(e.to_string())),
    }
}
