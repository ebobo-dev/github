use crate::domain::Device;
use crate::AppState;
use libsql::params;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use std::sync::Arc;

#[get("/admin")]
pub async fn index(state: &State<Arc<AppState>>) -> Result<Json<Vec<Device>>, BadRequest<String>> {
    let mut res = state
        .db
        .lock()
        .await
        .query("SELECT * FROM devices", params![])
        .await
        .unwrap();

    let mut devices = vec![];

    while let Some(device) = res.next().unwrap() {
        devices.push(Device {
            id: device.get(0).unwrap(),
            fingerprint: device.get(1).unwrap(),
            fighter: None,
        });
    }

    Ok(Json(devices))
}

#[post("/admin/reset")]
pub async fn reset(state: &State<Arc<AppState>>) -> Result<(), BadRequest<String>> {
    state
        .db
        .lock()
        .await
        .execute("DELETE FROM devices", params![])
        .await
        .unwrap();
    Ok(())
}
