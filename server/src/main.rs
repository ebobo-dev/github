mod domain;
mod fairings;
mod handlers;

#[macro_use]
extern crate rocket;

use fairings::*;
use handlers::*;
use libsql::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    db: Arc<Mutex<Connection>>,
}

#[shuttle_runtime::main]
async fn rocket(
    #[shuttle_turso::Turso(
        addr = "libsql://ebobo-dotnicht.turso.io",
        token = "{secrets.DB_TURSO_TOKEN}"
    )]
    db: Connection,
) -> shuttle_rocket::ShuttleRocket {
    db.execute_batch(
        "CREATE TABLE IF NOT EXISTS devices (
         id integer primary key autoincrement,
         fingerprint text not null unique,
         fighter text",
    )
    .await
    .unwrap();

    // db.execute_batch(
    //     "CREATE TABLE IF NOT EXISTS locations (
    //      id integer primary key autoincrement,
    //      address text not null unique)",
    // )
    // .await
    // .unwrap();

    // db.execute_batch(
    //     "CREATE TABLE IF NOT EXISTS devices_locations (
    //     device_id integer not null references devices(id),
    //     location_id integer not null references locations(id))",
    // )
    // .await
    // .unwrap();

    let state = AppState {
        db: Arc::new(Mutex::new(db)),
    };

    let rocket = rocket::build()
        .attach(cors::CORS)
        .mount("/", routes![auth::authenticate]) //, admin::reset, admin::index])
        .manage(state);

    Ok(rocket.into())
}
