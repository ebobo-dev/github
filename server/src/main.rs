mod domain;
mod fairings;
mod handlers;

#[macro_use]
extern crate rocket;

use fairings::*;
use handlers::*;
use shuttle_persist::PersistInstance;
use shuttle_secrets::SecretStore;

#[shuttle_runtime::main]
async fn rocket(
    #[shuttle_secrets::Secrets] _secrets: SecretStore,
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> shuttle_rocket::ShuttleRocket {
    let state = auth::AuthState { persist };

    let rocket = rocket::build()
        .attach(cors::CORS)
        .mount("/", routes![auth::authenticate, admin::reset, admin::index])
        .manage(state);

    Ok(rocket.into())
}
