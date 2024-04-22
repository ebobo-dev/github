use rocket::{response::status::BadRequest, serde::json::Json, serde::Serialize, State};

use crate::{guards::auth::Auth, AppState};

#[derive(Debug, Serialize)]
pub struct User<'a> {
    pub id: i32,
    pub name: &'a str,
    pub photo: &'a str,
    pub fingerprint: String,
}

#[options("/admin")]
pub async fn options() {}

#[get("/admin")]
pub async fn get(auth: Auth, _state: &State<AppState>) -> Result<Json<User>, BadRequest<String>> {
    let user = User {
        id: 0xEB0B0,
        name: "Huxm",
        photo: "🐱",
        fingerprint: auth.fingerprint
    };

    Ok(Json(user))
}
