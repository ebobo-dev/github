use std::sync::Arc;

use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::prelude::*;
use sea_orm::*;

use ebobo_shared::*;

use crate::entities::{devices::*, prelude::*};

#[post("/choose", data = "<request>")]
pub async fn choose(
    request: Json<Fighter>,
    state: &State<Arc<DatabaseConnection>>,
) -> Result<(), BadRequest<String>> {
    let device = Devices::find()
        .filter(Column::Fingerprint.eq(request.fingerprint.clone()))
        .one(state.as_ref())
        .await
        .map_err(|e| BadRequest(format!("Failed to find device: {}", e.to_string())))?;

    match device {
        Some(device) => {
            let device = ActiveModel {
                id: ActiveValue::unchanged(device.id),
                fingerprint: ActiveValue::unchanged(request.fingerprint.clone()),
                fighter: ActiveValue::set(request.fighter.clone()),
            };

            Devices::update(device)
                .exec(state.as_ref())
                .await
                .map_err(|e| BadRequest(format!("Failed to update device: {}", e.to_string())))?;

            Ok(())
        }
        None => Err(BadRequest("Failed to find device".to_string())),
    }
}
