mod cors;
mod auth;
mod index;
mod models;

#[macro_use]
extern crate rocket;

use rocket::State;
use shuttle_persist::PersistInstance;
use models::*;

#[shuttle_runtime::main]
async fn rocket(
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> shuttle_rocket::ShuttleRocket {
    let state = AuthState { persist };
    
    let rocket = rocket::build()
        .attach(cors::CORS)
        .mount("/", routes![auth::auth, index::index])
        .manage(state);

    Ok(rocket.into())
}
