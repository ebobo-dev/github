use crate::auth::AuthState;
use rocket::State;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use ebobo_shared::Device;

#[get("/admin")]
pub fn index(state: &State<AuthState>) -> Result<Json<Vec<Device>>, BadRequest<String>> {
    let devices = state.persist.list().unwrap().iter().map(|f| {
        state.persist.load::<Device>(f).unwrap()
    }).collect();
    Ok(Json(devices))
}

#[post("/admin/reset")]
pub fn reset(state: &State<AuthState>) -> Result<(), BadRequest<String>> {
    state.persist.clear().unwrap();
    Ok(())
}