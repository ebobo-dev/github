use rocket::{response::status::BadRequest, serde::json::Json, State};
use sea_orm::*;

use ebobo_shared::Arena;

use crate::{
    entities::{prelude::*, users},
    guards::fighter::Fighter,
    AppState,
};

#[options("/arena")]
pub async fn options() {}

#[get("/arena")]
pub async fn get(
    fighter: Fighter,
    state: &State<AppState>,
) -> Result<Json<Arena>, BadRequest<String>> {
    Ok(Json(Arena {
        total: 0,
        queue: 0,
        rank: 0,
        you: fighter.fighter,
    }))
}
