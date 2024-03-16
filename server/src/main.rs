mod entities;
mod fairings;
mod guards;
mod handlers;

#[macro_use]
extern crate rocket;

use fairings::*;
use handlers::*;
use sea_orm::*;
use shuttle_secrets::SecretStore;
use std::sync::Arc;

#[shuttle_runtime::main]
async fn rocket(#[shuttle_secrets::Secrets] secrets: SecretStore) -> shuttle_rocket::ShuttleRocket {
    let conn = Database::connect(
        secrets
            .get("DB_CONNECTION_STRING")
            .expect("Connection string not found"),
    )
    .await
    .expect("Failed to connect to the database");

    let rocket = rocket::build()
        .manage(Arc::new(conn))
        .attach(cors::CORS)
        .mount("/", routes![auth::authenticate, fight::choose,]);

    Ok(rocket.into())
}
