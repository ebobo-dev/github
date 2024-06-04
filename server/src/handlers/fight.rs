use ebobo_shared::Utc;
use rocket::{response::status::BadRequest, State};
use sea_orm::{prelude::*, *};

use crate::{
    entities::{prelude::*, queue::ActiveModel},
    guards::fighter::Fighter,
    EboboState,
};

#[options("/fight")]
pub async fn options() {}

#[post("/fight")]
pub async fn post(auth: Fighter, state: &State<EboboState>) -> Result<(), BadRequest<String>> {
    Queue::insert(ActiveModel {
        id: ActiveValue::set(Uuid::new_v4()),
        fighter: ActiveValue::set(auth.fingerprint),
        date: ActiveValue::set(Utc::now().naive_utc()),
    })
    .exec(state.db.as_ref())
    .await
    .map_err(|e| BadRequest(format!("failed to queue for matchmaking: {}", e.to_string())))?;
    Ok(())
}
