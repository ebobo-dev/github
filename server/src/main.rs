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

use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{CreateCollection, VectorParams, VectorsConfig};

#[shuttle_runtime::main]
async fn rocket(
    #[shuttle_secrets::Secrets] _secrets: SecretStore,
    #[shuttle_persist::Persist] persist: PersistInstance,
    #[shuttle_qdrant::Qdrant(
        cloud_url = "https://25197a8d-dcf0-497d-97e2-ce57cdbdae57.us-east4-0.gcp.cloud.qdrant.io",
        api_key = "{secrets.DB_QDRANT_TOKEN}"
    )]
    qdrant: QdrantClient,
) -> shuttle_rocket::ShuttleRocket {
    let state = auth::AuthState { persist };

    let rocket = rocket::build()
        .attach(cors::CORS)
        .mount("/", routes![auth::authenticate, admin::reset, admin::index])
        .manage(state);

    qdrant
        .create_collection(&CreateCollection {
            collection_name: "test".to_owned(),
            vectors_config: Some(VectorsConfig {
                config: Some(Config::Params(VectorParams {
                    size: 10,
                    distance: Distance::Cosine.into(),
                    ..Default::default()
                })),
            }),
            ..Default::default()
        })
        .await?;

    Ok(rocket.into())
}
