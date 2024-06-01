use rocket::{response::status::BadRequest, serde::json::Json, State};
use sea_orm::*;

use ebobo_shared::Arena;

use crate::{
    entities::{prelude::*, users},
    guards::auth::Auth,
    AppState,
};

#[options("/arena")]
pub async fn options() {}

#[get("/arena")]
pub async fn get(auth: Auth, state: &State<AppState>) -> Result<Json<Arena>, BadRequest<String>> {
    Ok(Json(Arena {
        total: 0,
        ready: 0,
    }))
}
