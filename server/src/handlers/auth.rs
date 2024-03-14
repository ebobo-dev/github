use ebobo_shared::*;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[post("/authenticate", data = "<request>")]
pub async fn authenticate(
    request: Json<Auth>,
    state: &State<Arc<DatabaseConnection>>,
) -> Result<Json<Fighter>, BadRequest<String>> {
    
        
    panic!()
}
