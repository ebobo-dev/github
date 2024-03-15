use crate::entities::{devices::*, prelude::*};
use ebobo_shared::*;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::*;
use std::sync::Arc;

#[post("/authenticate", data = "<request>")]
pub async fn authenticate(
    request: Json<Auth>,
    state: &State<Arc<DatabaseConnection>>,
) -> Result<Json<Fighter>, BadRequest<String>> {
    let device = Devices::find()
        .filter(Column::Fingerprint.eq(request.fingerprint.clone()))
        .one(state.as_ref())
        .await
        .unwrap();

    if device.is_none() {
        let device = ActiveModel {
            fingerprint: ActiveValue::set(request.fingerprint.clone()),
            ..Default::default()
        };

        Devices::insert(device).exec(state.as_ref()).await.unwrap();
    }

    Ok(Json(Fighter {
        fingerprint: request.fingerprint.clone(),
        fighter: None,
    }))
}
