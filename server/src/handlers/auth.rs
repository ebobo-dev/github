use crate::domain::*;
use ebobo_shared::*;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;

#[post("/authenticate", data = "<request>")]
pub fn authenticate(
    request: Json<Auth>,
) -> Result<String, BadRequest<String>> {
    Ok("Hello, world!".to_owned())
}
