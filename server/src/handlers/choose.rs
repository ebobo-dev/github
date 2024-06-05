use rocket::{response::status::BadRequest, serde::json::Json, State};
use sea_orm::{prelude::*, *};

use crate::{
    entities::{prelude::*, fighters::*},
    EboboState,
};

#[options("/choose")]
pub async fn options() {}

#[post("/choose", data = "<request>")]
pub async fn post(
    auth: crate::auth::Auth,
    state: &State<EboboState>,
    request: Json<ebobo_shared::Choice>,
) -> Result<(), BadRequest<String>> {
    let user = ActiveModel {
        fingerprint: ActiveValue::set(auth.fingerprint),
        rank: Default::default(),
        emo: ActiveValue::set(request.0 .0),
    };

    Fighters::insert(user)
        .exec(state.db.as_ref())
        .await
        .map_err(|e| BadRequest(format!("Failed to insert user: {}", e.to_string())))?;

    Ok(())
}
