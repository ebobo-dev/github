use rocket::{response::status::BadRequest, serde::json::Json, State};
use sea_orm::*;

use crate::{
    entities::{prelude::*, users},
    guards::auth::Auth,
    AppState,
};

#[options("/")]
pub async fn options() {}

#[get("/")]
pub async fn post(auth: Auth, state: &State<AppState>) -> Result<(), BadRequest<String>> {
    Ok(())
}
