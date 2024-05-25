use rocket::{response::status::BadRequest, serde::json::Json, State};
use sea_orm::*;

use ebobo_shared::{Fighter, Index};

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

    let fighters = vec!['🐱', '🐵', '🐶', '🐷', '🐰', '🐮'];

    let taken = Users::find()
        .column(users::Column::Fingerprint)
        .all(state.db.as_ref())
        .await
        .map_err(|e| BadRequest(e.to_string()))?
        .into_iter()
        .map(|f| f.fighter)
        .collect::<Vec<String>>();

    let available = fighters
        .into_iter()
        .filter(|f| !taken.contains(&f.to_string()))
        .map(|f| Fighter::new(f.to_string().as_str()))
        .collect();

    Ok(Json(Index {
        fighter: user.is_some(),
        root: user.is_some() && user.unwrap().root,
        greet,
        fighters: available,
    }))
}
