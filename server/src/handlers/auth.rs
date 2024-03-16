use crate::entities::{devices::*, prelude::*};
use crate::guards::device::Device;
use ebobo_shared::*;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::*;
use shuttle_secrets::SecretStore;
use std::sync::Arc;

#[post("/authenticate", data = "<request>")]
pub async fn authenticate(
    request: Json<Auth>,
    db: &State<Arc<DatabaseConnection>>,
    secrets: &State<Arc<SecretStore>>,
    device: Device,
) -> Result<Json<Fighter>, BadRequest<String>> {
    let secret = secrets.get("ADMIN_FINGERPRINT").unwrap();
    if device.fingerprint != secret {
        return Err(BadRequest("Unauthorized".to_string()));
    }

    let existing = Devices::find()
        .filter(Column::Fingerprint.eq(request.fingerprint.clone()))
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
                fingerprint: ActiveValue::set(request.fingerprint.clone()),
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
