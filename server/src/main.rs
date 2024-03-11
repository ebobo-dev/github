mod domain;
mod fairings;
mod handlers;

#[macro_use]
extern crate rocket;

use fairings::*;
use handlers::*;
use qdrant_client::prelude::*;
use shuttle_persist::PersistInstance;
use shuttle_secrets::SecretStore;

#[shuttle_runtime::main]
async fn rocket(
    #[shuttle_secrets::Secrets] _secrets: SecretStore,
    #[shuttle_persist::Persist] persist: PersistInstance,
    #[shuttle_qdrant::Qdrant(
        cloud_url = "https://25197a8d-dcf0-497d-97e2-ce57cdbdae57.us-east4-0.gcp.cloud.qdrant.io",
        api_key = "{secrets.DB_QDRANT_TOKEN}"
    )]
    _qdrant: QdrantClient,
) -> shuttle_rocket::ShuttleRocket {
    let state = auth::AuthState { persist };

    let rocket = rocket::build()
        .attach(cors::CORS)
        .mount("/", routes![auth::authenticate, admin::reset, admin::index])
        .manage(state);

    Ok(rocket.into())
}
