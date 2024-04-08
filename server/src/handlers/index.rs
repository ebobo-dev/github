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

    let greet = match user {
        Some(user) => format!("Hi, {}!", user.fighter),
        None => format!("Hello, {}!", auth.fingerprint),
    };

    Ok(Json(Index {
        greet,
        fighters: vec!["🐱", "🐵", "🐶", "🐷"]
            .into_iter()
            .map(|f| Fighter::new(f))
            .collect(),
    }))
}
