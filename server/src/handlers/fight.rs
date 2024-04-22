use rocket::{response::status::BadRequest, State, serde::json::Json};
use sea_orm::{prelude::*, *};

use ebobo_shared::Choice;

use crate::{
    entities::{prelude::*, users::*},
    guards::auth::Auth,
    AppState,
};

#[options("/choose")]
pub async fn options() {}

#[post("/choose", data = "<request>")]
pub async fn post(
    auth: Auth,
    request: Json<Choice>,
    state: &State<AppState>,
) -> Result<(), BadRequest<String>> {
    let count = Users::find()
        .all(state.db.as_ref())
        .await
        .map_err(|e| BadRequest(format!("Failed to fetch users count: {}", e)))?
        .iter()
        .count();

    let user = ActiveModel {
        id: ActiveValue::set(Uuid::new_v4()),
        fingerprint: ActiveValue::set(auth.fingerprint.clone()),
        rank: Default::default(),
        root: ActiveValue::set(count == 0),
        fighter: ActiveValue::set(request.fighter()),
    };

    Users::insert(user)
        .exec(state.db.as_ref())
        .await
        .map_err(|e| BadRequest(format!("Failed to insert user: {}", e.to_string())))?;

    Ok(())
}
