use rocket::{response::status::BadRequest, serde::json::Json, State};
use sea_orm::{prelude::*, *};

use crate::{
    entities::{prelude::*, users::*},
    guards::auth::Auth,
    EboboState,
};

#[options("/choose")]
pub async fn options() {}

#[post("/choose", data = "<request>")]
pub async fn post(
    auth: Auth,
    state: &State<EboboState>,
    request: Json<ebobo_shared::Choice>,
) -> Result<(), BadRequest<String>> {
    let user = ActiveModel {
        id: ActiveValue::set(Uuid::new_v4()),
        fingerprint: ActiveValue::set(auth.fingerprint),
        rank: Default::default(),
        fighter: ActiveValue::set(request.0 .0),
    };

    Users::insert(user)
        .exec(state.db.as_ref())
        .await
        .map_err(|e| BadRequest(format!("Failed to insert user: {}", e.to_string())))?;

    Ok(())
}
