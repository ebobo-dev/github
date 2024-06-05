use rocket::{response::status::BadRequest, serde::json::Json, State};
use sea_orm::*;

use ebobo_shared::Index;

use crate::{
    entities::{prelude::*, users},
    EboboState,
};

#[options("/")]
pub async fn options() {}

#[get("/")]
pub async fn get(auth: crate::auth::Auth, state: &State<EboboState>) -> Result<Json<Index>, BadRequest<String>> {
    match auth.fighter {
        Some(user) => Ok(Json(Index {
            greet: format!("hi, {}! your rank is {}.", user.emo, user.rank),
            fighter: Some(user.emo),
            rank: Some(user.rank),
        })),
        None => Ok(Json(Index {
            greet: format!("hello, {}!", auth.fingerprint),
            fighter: None,
            rank: None,
        })),
    }
}
