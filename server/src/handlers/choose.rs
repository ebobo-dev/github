use ebobo_shared::Choice;
use rocket::{response::status::BadRequest, serde::json::Json, State};
use sea_orm::{prelude::*, *};

use crate::{
    entities::{prelude::*, users::*},
    guards::auth::Auth,
    AppState,
};

#[options("/fight")]
pub async fn options() {}

#[post("/fight", data = "<request>")]
pub async fn post(
    auth: Auth,
    state: &State<AppState>,
    request: Json<Choice>,
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
