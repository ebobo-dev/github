use crate::AppState;
use ebobo_shared::*;
use libsql::params;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use std::sync::Arc;

#[post("/authenticate", data = "<request>")]
pub async fn authenticate(
    request: Json<Auth>,
    state: &State<Arc<AppState>>,
) -> Result<String, BadRequest<String>> {
    let mut res = state
        .db
        .lock()
        .await
        .query(
            "SELECT * FROM devices WHERE fingerprint = ?1",
            params!(request.fingerprint.to_owned()),
        )
        .await
        .unwrap();

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
                    .unwrap();
                Ok("inserted".to_string())
            },
            Ok(Some(_)) => Ok("will update".to_string()),
            Err(e) => Err(BadRequest(e.to_string())),
        }
        
}
