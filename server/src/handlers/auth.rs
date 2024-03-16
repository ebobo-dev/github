use crate::entities::{devices::*, prelude::*};
use crate::guards::device::Device;
use ebobo_shared::*;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::*;
use std::sync::Arc;

#[post("/authenticate")]
pub async fn authenticate(
    db: &State<Arc<DatabaseConnection>>,
    device: Device,
) -> Result<Json<Fighter>, BadRequest<String>> {
    let existing = Devices::find()
        .filter(Column::Fingerprint.eq(device.fingerprint.clone()))
        .one(db.as_ref())
        .await
        .map_err(|e| BadRequest(format!("Failed to find device: {}", e.to_string())))?;

    match existing {
        Some(d) => {
            return Ok(Json(Fighter {
                fingerprint: d.fingerprint,
                fighter: d.fighter,
            }));
        }
        None => {
            let created = ActiveModel {
                fingerprint: ActiveValue::set(device.fingerprint.clone()),
                ..Default::default()
            };

            let result: InsertResult<ActiveModel> = Devices::insert(created)
                .exec(db.as_ref())
                .await
                .map_err(|e| BadRequest(format!("Failed to insert device: {}", e.to_string())))?;

            let created = Devices::find()
                .filter(Column::Id.eq(result.last_insert_id))
                .one(db.as_ref())
                .await
                .map_err(|e| BadRequest(format!("Failed to find device: {}", e.to_string())))?;

            match created {
                Some(d) => Ok(Json(Fighter {
                    fingerprint: d.fingerprint,
                    fighter: d.fighter,
                })),
                None => Err(BadRequest("Failed to find created device".to_string())),
            }
        }
    }
}
