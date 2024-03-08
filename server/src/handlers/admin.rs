use crate::auth::AuthState;
use rocket::State;
use rocket::response::status::BadRequest;

#[post("/reset")]
pub fn reset(state: &State<AuthState>) -> Result<(), BadRequest<String>> {
    state.persist.clear().unwrap();
    Ok(())
}