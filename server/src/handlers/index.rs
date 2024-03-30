use rocket::{response::status::BadRequest, serde::json::Json, State};
use sea_orm::*;

use ebobo_shared::*;

use crate::{entities::prelude::*, guards::auth::Auth, AppState};

#[options("/")]
pub async fn options() {}

#[get("/")]
pub async fn get(
    _auth: Auth,
    state: &State<AppState>,
) -> Result<Json<Vec<Fighter>>, BadRequest<String>> {
    let fighters = Users::find()
        .all(state.db.as_ref())
        .await
        .map_err(|e| BadRequest(format!("Failed to fetch users: {}", e)))?
        .into_iter()
        .map(|device| Fighter {
            fingerprint: device.fingerprint,
            fighter: device.fighter,
        })
        .collect::<Vec<Fighter>>();

    Ok(Json(fighters))
}
