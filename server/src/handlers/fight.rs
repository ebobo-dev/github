use rocket::{response::status::BadRequest, State};
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
    request: String,
    state: &State<AppState>,
) -> Result<(), BadRequest<String>> {
    let device = Fighters::find()
        .filter(Column::Device.eq(auth.fingerprint.clone()))
        .one(state.db.as_ref())
        .await
        .map_err(|e| BadRequest(format!("Failed to find user: {}", e)))?;

    match device {
        Some(device) => {
            let device = ActiveModel {
                id: ActiveValue::unchanged(device.id),
                device: ActiveValue::unchanged(device.device),
                rank: ActiveValue::unchanged(device.rank),
                root: ActiveValue::unchanged(device.root),
                created: ActiveValue::unchanged(device.created),
                fighter: ActiveValue::set(Some(request)),
            };

            Fighters::update(device)
                .exec(state.db.as_ref())
                .await
                .map_err(|e| BadRequest(format!("Failed to update user: {}", e)))?;

            Ok(())
        }
        None => {
            let count = Fighters::find()
                .all(state.db.as_ref())
                .await
                .map_err(|e| BadRequest(format!("Failed to fetch users count: {}", e)))?
                .into_iter()
                .count();

            let device = ActiveModel {
                id: Default::default(),
                device: ActiveValue::set(auth.fingerprint.clone()),
                rank: Default::default(),
                root: ActiveValue::set(count == 0),
                created: ActiveValue::set(Utc::now().naive_utc()),
                fighter: ActiveValue::set(Some(request)),
            };

            Fighters::insert(device)
                .exec(state.db.as_ref())
                .await
                .map_err(|e| BadRequest(format!("Failed to insert user: {}", e.to_string())))?;

            Ok(())
        }
    }
}
