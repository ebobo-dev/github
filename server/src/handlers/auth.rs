use ebobo_shared::*;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
use std::sync::Arc;
use crate::entities::{devices::*, prelude::*};

#[post("/authenticate", data = "<request>")]
pub async fn authenticate(
    request: Json<Auth>,
    state: &State<Arc<DatabaseConnection>>,
) -> Result<Json<Fighter>, BadRequest<String>> {
    let device = ActiveModel {
        fingerprint: ActiveValue::set(request.fingerprint.clone()),
        ..Default::default()
    };

    Devices::insert(device).exec(state.as_ref()).await.unwrap();

    Ok(Json(Fighter {
        fingerprint: request.fingerprint.clone(),
        fighter: None,
    }))
}
