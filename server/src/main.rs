mod domain;
mod fairings;
mod handlers;

#[macro_use]
extern crate rocket;

use fairings::*;
use handlers::*;
use libsql::Connection;
use shuttle_secrets::SecretStore;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct DatabaseService {
    pub conn: Arc<Mutex<Connection>>,
}

#[shuttle_runtime::main]
async fn rocket(
    #[shuttle_turso::Turso(addr = "{secrets.TURSO_ADDR}", token = "{secrets.TURSO_TOKEN}")]
    db: Connection,
) -> shuttle_rocket::ShuttleRocket {
    db.execute_batch(
        "CREATE TABLE IF NOT EXISTS devices (
         id integer primary key autoincrement,
         fingerprint text not null,
         fighter text null)",
    )
    .await
    .unwrap();

    db.execute_batch(
        "CREATE TABLE IF NOT EXISTS locations (
         id integer primary key autoincrement,
         address text not null)",
    )
    .await
    .unwrap();

    let service = DatabaseService {
        conn: Arc::new(Mutex::new(db)),
    };

    let rocket = rocket::build()
        .attach(cors::CORS)
        .mount("/", routes![auth::authenticate])//, admin::reset, admin::index])
        .manage(service);

    Ok(rocket.into())
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for DatabaseService {
    async fn bind(mut self, addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        Ok(())
    }
}
