use crate::entities::{devices::*, prelude::*};
use crate::guards::device::Device;
use ebobo_shared::*;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::*;
use std::sync::Arc;

#[options("/authenticate")]
pub async fn options() {
    
}

#[post("/authenticate")]
pub async fn authenticate(
    device: Device,
    state: &State<Arc<DatabaseConnection>>,
) -> Result<Json<Fighter>, BadRequest<String>> {
    let existing = Devices::find()
        .filter(Column::Fingerprint.eq(device.fingerprint.clone()))
        .one(state.as_ref())
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
            let device = ActiveModel {
                fingerprint: ActiveValue::set(device.fingerprint.clone()),
                ..Default::default()
            };
    
            let result: InsertResult<ActiveModel> = Devices::insert(device)
                .exec(state.as_ref())
                .await
                .map_err(|e| BadRequest(format!("Failed to insert device: {}", e.to_string())))?;
    
            let device = Devices::find()
                .filter(Column::Id.eq(result.last_insert_id))
                .one(state.as_ref())
                .await
                .map_err(|e| BadRequest(format!("Failed to find device: {}", e.to_string())))?;
    
            match device {
                Some(device) => {
                    Ok(Json(Fighter {
                        fingerprint: device.fingerprint,
                        fighter: device.fighter,
                    }))
                }
                None => {
                    Err(BadRequest("Failed to find created device".to_string()))
                }
            }
        }
    }    
}