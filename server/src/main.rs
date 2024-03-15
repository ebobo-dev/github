mod domain;
mod entities;
mod fairings;
mod handlers;

#[macro_use]
extern crate rocket;

use fairings::*;
use handlers::*;
use sea_orm::*;
use shuttle_secrets::SecretStore;
use std::sync::Arc;

#[shuttle_runtime::main]
async fn rocket(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_rocket::ShuttleRocket {
    let conn = Database::connect(secret_store.get("DB_CONNECTION_STRING").unwrap())
        .await
        .unwrap();

    let rocket = rocket::build()
        .manage(Arc::new(conn))
        .attach(cors::CORS)
        .mount(
            "/",
            routes![
                auth::authenticate,
                fight::choose,
                admin::reset,
                admin::index
            ],
        );

    Ok(rocket.into())
}
