use rocket::{response::status::BadRequest, serde::json::Json, State};
use sea_orm::*;

use ebobo_shared::Arena;

use crate::{
    entities::{fighters, prelude::*},
    EboboState,
};

#[options("/arena")]
pub async fn options() {}

#[get("/arena")]
pub async fn get(
    fighter: crate::auth::Auth,
    state: &State<EboboState>,
) -> Result<Json<Arena>, BadRequest<String>> {
    let total = Fighters::find().all(state.db.as_ref()).await.unwrap().len() as i32; // TODO: unwrap
    let queue = Fighters::find()
        .filter(fighters::Column::Queued.eq(true))
        .all(state.db.as_ref())
        .await
        .unwrap()
        .len() as i32;

    let fighter = fighter.fighter.unwrap();

    Ok(Json(Arena {
        total,
        queue,
        rank: fighter.rank as i32,
        you: fighter.emo,
    }))
}
