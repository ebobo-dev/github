use rocket::{response::status::BadRequest, serde::json::Json, State};
use sea_orm::*;

use crate::{
    entities::{prelude::*, users},
    guards::auth::Auth,
    EboboState,
};

#[options("/fight")]
pub async fn options() {}

#[post("/fight")]
pub async fn post(auth: Auth, state: &State<EboboState>) -> Result<(), BadRequest<String>> {
    Ok(())
}
