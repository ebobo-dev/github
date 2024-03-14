use crate::domain::Device;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[get("/admin")]
pub async fn index(
    state: &State<Arc<DatabaseConnection>>,
) -> Result<Json<Vec<Device>>, BadRequest<String>> {
    panic!()
}

#[post("/admin/reset")]
pub async fn reset(state: &State<Arc<DatabaseConnection>>) -> Result<(), BadRequest<String>> {
    Ok(())
}
