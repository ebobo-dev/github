mod domain;
mod fairings;
mod handlers;

#[macro_use]
extern crate rocket;

use fairings::*;
use handlers::*;
use sea_orm::*;
use std::sync::Arc;

#[shuttle_runtime::main]
async fn rocket(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> shuttle_rocket::ShuttleRocket {
    let conn = Database::connect("postgresql://neondb_owner:L5EhgjFndt6R@ep-royal-math-a2j2dztz-pooler.eu-central-1.aws.neon.tech/neondb?sslmode=require");

    let rocket = rocket::build()
        .attach(cors::CORS)
        .mount(
            "/",
            routes![
                auth::authenticate,
                fight::choose,
                admin::reset,
                admin::index
            ],
        )
        .manage(Arc::new(conn));

    Ok(rocket.into())
}
