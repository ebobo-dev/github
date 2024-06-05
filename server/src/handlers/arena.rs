use rocket::{response::status::BadRequest, serde::json::Json, State};
use sea_orm::*;

use ebobo_shared::Arena;

use crate::{entities::prelude::*, guards::fighter::Fighter, EboboState};

#[options("/arena")]
pub async fn options() {}

#[get("/arena")]
pub async fn get(
    fighter: Fighter,
    state: &State<EboboState>,
) -> Result<Json<Arena>, BadRequest<String>> {
    let total = Users::find().all(state.db.as_ref()).await.unwrap().len();
    let queue = Queue::find().all(state.db.as_ref()).await.unwrap().len();

    Ok(Json(Arena {
        total: total,
        queue: queue,
        rank: fighter.rank,
        you: fighter.fighter,
    }))
}
