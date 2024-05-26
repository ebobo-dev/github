use rocket::{response::status::BadRequest, serde::json::Json, State};
use sea_orm::*;

use ebobo_shared::Index;

use crate::{
    entities::{prelude::*, users},
    guards::auth::Auth,
    AppState,
};

#[options("/")]
pub async fn options() {}

#[get("/")]
pub async fn get(auth: Auth, state: &State<AppState>) -> Result<Json<Index>, BadRequest<String>> {
    let user = Users::find()
        .filter(users::Column::Fingerprint.eq(&auth.fingerprint))
        .one(state.db.as_ref())
        .await
        .map_err(|e| BadRequest(e.to_string()))?;

    let greet = match user.clone() {
        Some(user) => format!("hi, {}! your rank is {}.", user.fighter, user.rank),
        None => format!("hello, {}!", auth.fingerprint),
    };

    Ok(Json(Index {
        fighter: user.is_some(),
        root: user.is_some() && user.unwrap().root,
        greet,
    }))
}
