use crate::entities::prelude::*;
use ebobo_shared::*;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::prelude::*;
use std::sync::Arc;

#[post("/choose", data = "<request>")]
pub async fn choose(
    request: Json<Fighter>,
    state: &State<Arc<DatabaseConnection>>,
) -> Result<(), BadRequest<String>> {
    

    Ok(())
}
