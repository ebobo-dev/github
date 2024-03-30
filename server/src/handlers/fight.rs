use rocket::{response::status::BadRequest, State};
use sea_orm::*;

use crate::{
    entities::{users::*, prelude::*},
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
    let device = Users::find()
        .filter(Column::Fingerprint.eq(auth.fingerprint.clone()))
        .one(state.db.as_ref())
        .await
        .map_err(|e| BadRequest(format!("Failed to find user: {}", e)))?;

    match device {
        Some(device) => {
            let device = ActiveModel {
                id: ActiveValue::unchanged(device.id),
                fingerprint: ActiveValue::unchanged(device.fingerprint),
                rank: ActiveValue::unchanged(device.rank),
                root: ActiveValue::unchanged(device.root),
                fighter: ActiveValue::set(Some(request)),
            };

            Users::update(device)
                .exec(state.db.as_ref())
                .await
                .map_err(|e| BadRequest(format!("Failed to update user: {}", e)))?;

            Ok(())
        }
        None => {
            let count = Users::find()
                .all(state.db.as_ref())
                .await
                .map_err(|e| BadRequest(format!("Failed to fetch users count: {}", e)))?
                .into_iter()
                .count();

            let device = ActiveModel {
                id: Default::default(),
                fingerprint: ActiveValue::set(auth.fingerprint.clone()),
                rank: Default::default(),
                root: ActiveValue::set(count == 0),
                fighter: ActiveValue::set(Some(request)),
            };

            Users::insert(device)
                .exec(state.db.as_ref())
                .await
                .map_err(|e| BadRequest(format!("Failed to insert user: {}", e.to_string())))?;

            Ok(())
        }
    }
}
