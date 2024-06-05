#[macro_use]
extern crate rocket;

use std::sync::Arc;
use sea_orm::*;
use serde::Deserialize;

use handlers::*;

mod entities;
mod cors;
mod auth;
mod handlers;

pub struct EboboState {
    pub db: Arc<DatabaseConnection>,
    pub secrets: shuttle_runtime::SecretStore,
}

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
        .manage(EboboState {
            db: Arc::new(conn),
            secrets,
        })
        .attach(cors::CORS)
        .mount(
            "/",
            routes![
                index::options,
                index::get,
                choose::options,
                choose::post,
                available::options,
                available::get,
                arena::options,
                arena::get,
                fight::options,
                fight::post
            ],
        );

    Ok(rocket.into())
}
