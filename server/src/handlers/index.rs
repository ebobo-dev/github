use std::sync::Arc;

use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::*;

use ebobo_shared::*;
use serde::de;

use crate::entities::prelude::*;
use crate::guards::auth::Auth;

// #[options("/")]
// pub async fn options() {}

#[get("/")]
pub async fn get(
    auth: Auth,
    state: &State<Arc<DatabaseConnection>>,
) -> Result<Json<Vec<Fighter>>, BadRequest<String>> {
    let devices = Devices::find()
        .all(state.as_ref())
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
