use crate::AppState;
use ebobo_shared::*;
use libsql::params;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use std::sync::Arc;

#[post("/choose", data = "<request>")]
pub async fn choose(
    request: Json<Fighter>,
    state: &State<Arc<AppState>>,
) -> Result<(), BadRequest<String>> {
    let c =state
        .db
        .lock()
        .await
        .execute(
            "UPDATE devices SET fighter = ?1 WHERE fingerprint = ?2",
            params!(request.fighter.to_owned(), request.fingerprint.to_owned()),
        )
        .await
        .map_err(|e| BadRequest(e.to_string()))?;

        if c == 0 {
            panic!("no rows updated")
        }

    Ok(())
}
