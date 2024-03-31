use rocket::{response::status::BadRequest, State};
use sea_orm::*;

use crate::{
    entities::{prelude::*, users},
    guards::auth::Auth,
    AppState,
};

#[options("/")]
pub async fn options() {}

#[get("/")]
pub async fn get(auth: Auth, state: &State<AppState>) -> Result<String, BadRequest<String>> {
    let user = Users::find()
        .filter(users::Column::Fingerprint.eq(&auth.fingerprint))
        .one(state.db.as_ref())
        .await
        .map_err(|e| BadRequest(e.to_string()))?;

    let greet = match user {
        Some(user) => {
            format!("Hi, {}!", user.fighter)
        },
        None => {
            format!("Hello, {}!", auth.fingerprint)
        }
    };

    Ok(greet)
}