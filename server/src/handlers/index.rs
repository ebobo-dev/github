use rocket::{response::status::BadRequest, serde::json::Json, State};
use sea_orm::*;

use ebobo_shared::*;

use crate::{entities::{fighters, prelude::*}, guards::auth::Auth, AppState};

#[options("/")]
pub async fn options() {}

#[get("/")]
pub async fn get(
    _auth: Auth,
    state: &State<AppState>,
) -> Result<Json<Vec<Fighter>>, BadRequest<String>> {
    let fighters = Fighters::find()
        .all(state.db.as_ref())
        .await
        .map_err(|e| BadRequest(format!("Failed to find device location: {}", e.to_string())))?
        .into_iter()
        .map(|device| Fighter {
            fingerprint: device.device,
            fighter: device.fighter,
        })
        .collect::<Vec<Fighter>>();

    Ok(Json(fighters))
}
