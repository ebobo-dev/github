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
    let devices = Devices::find()
        .all(state.db.as_ref())
        .await
        .map_err(|e| BadRequest(format!("Failed to find device location: {}", e.to_string())))?
        .into_iter()
        .map(|device| Fighter {
            fingerprint: device.fingerprint,
            fighter: device.fighter,
        })
        .collect::<Vec<Fighter>>();

    Ok(Json(devices))
}
