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
    
    match user {
        Some(user) => Ok(Json(Index {
            greet: format!("hi, {}! your rank is {}.", user.fighter, user.rank),
            fighter: Some(user.fighter),
            rank: Some(user.rank),
        })),
        None => Ok(Json(Index {
            greet: format!("hello, {}!", auth.fingerprint),
            fighter: None,
            rank: None,
        })),
    }
}
