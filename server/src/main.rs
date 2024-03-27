#[macro_use]
extern crate rocket;

use std::sync::Arc;

use sea_orm::*;

use fairings::*;
use handlers::*;

mod entities;
mod fairings;
mod guards;
mod handlers;

#[shuttle_runtime::main]
async fn rocket(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_rocket::ShuttleRocket {
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
        .mount(
            "/",
            routes![
                index::options,
                index::get,
                auth::options,
                auth::authenticate,
                fight::options,
                fight::choose
            ],
        );

    Ok(rocket.into())
}
