use rocket::{response::status::BadRequest, serde::json::Json, State};
use sea_orm::*;

use ebobo_shared::*;

use crate::{
    entities::{fighters::*, prelude::*},
    guards::auth::Auth,
    AppState,
};

#[options("/choose")]
pub async fn options() {}

#[post("/choose", data = "<request>")]
pub async fn choose(
    auth: Auth,
    request: Json<Fighter>,
    state: &State<AppState>,
) -> Result<(), BadRequest<String>> {
    let device = Fighters::find()
        .filter(Column::Device.eq(auth.fingerprint))
        .one(state.db.as_ref())
        .await
        .map_err(|e| BadRequest(format!("Failed to find device: {}", e.to_string())))?;

    match device {
        Some(device) => {
            let device = ActiveModel {
                id: ActiveValue::unchanged(device.id),
                device: ActiveValue::unchanged(device.device),
                rank: ActiveValue::unchanged(device.rank),
                root: ActiveValue::unchanged(device.root),
                created: ActiveValue::unchanged(device.created),
                fighter: ActiveValue::set(request.fighter.clone()),
            };

            Fighters::update(device)
                .exec(state.db.as_ref())
                .await
                .map_err(|e| BadRequest(format!("Failed to update device: {}", e.to_string())))?;

            Ok(())
        }
        None => {
            let device = ActiveModel {
                id: Default::default(),
                device: ActiveValue::set(request.fingerprint.clone()),
                rank: Default::default(),
                root: Default::default(),
                created: ActiveValue::set(Utc::now().naive_utc()),
                fighter: ActiveValue::set(request.fighter.clone()),
            };

            Fighters::insert(device)
                .exec(state.db.as_ref())
                .await
                .map_err(|e| BadRequest(format!("Failed to insert device: {}", e.to_string())))?;

            Ok(())
        }
    }
}
