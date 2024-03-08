mod cors;
mod auth;
mod domain;

#[macro_use]
extern crate rocket;

use shuttle_persist::PersistInstance;
use auth::AuthState;

#[shuttle_runtime::main]
async fn rocket(
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> shuttle_rocket::ShuttleRocket {
    let state = AuthState { persist };
    
    let rocket = rocket::build()
        .attach(cors::CORS)
        .mount("/", routes![auth::auth])
        .manage(state);

    Ok(rocket.into())
}
