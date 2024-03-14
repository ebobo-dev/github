use crate::domain::Device;
use crate::entities::prelude::*;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::sync::Arc;

#[get("/admin")]
pub async fn index(
    state: &State<Arc<DatabaseConnection>>,
) -> Result<Json<Vec<Device>>, BadRequest<String>> {
    let devices = Devices::find().all(state.as_ref()).await.unwrap();

    let devices = devices
        .into_iter()
        .map(|device| Device {
            fingerprint: device.fingerprint,
            fighter: None,
        })
        .collect::<Vec<Device>>();

    Ok(Json(devices))
}

#[post("/admin/reset")]
pub async fn reset(state: &State<Arc<DatabaseConnection>>) -> Result<(), BadRequest<String>> {
    panic!("Not implemented");
    Ok(())
}
