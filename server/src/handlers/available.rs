use rocket::{response::status::BadRequest, serde::json::Json, State};
use sea_orm::*;

use ebobo_shared::Fighter;

use crate::{
    entities::{prelude::*, users},
    guards::auth::Auth,
    EboboState,
};

#[options("/available")]
pub async fn options() {}

#[get("/available")]
pub async fn get(
    _auth: Auth,
    state: &State<EboboState>,
) -> Result<Json<Vec<Fighter>>, BadRequest<String>> {
    let fighters = ['🐱', '🐵', '🐶', '🐷', '🐰', '🐮'];

    let taken = Users::find()
        .column(users::Column::Fingerprint)
        .all(state.db.as_ref())
        .await
        .map_err(|e| BadRequest(e.to_string()))?
        .into_iter()
        .map(|f| f.fighter)
        .collect::<Vec<String>>();

    Ok(Json(
        fighters
            .into_iter()
            .filter(|f| !taken.contains(&f.to_string()))
            .map(|f| Fighter(f.to_string()))
            .collect(),
    ))
}
