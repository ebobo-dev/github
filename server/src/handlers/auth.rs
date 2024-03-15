use crate::entities::{devices::*, prelude::*};
use ebobo_shared::*;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::*;
use std::sync::Arc;

#[post("/authenticate", data = "<request>")]
pub async fn authenticate(
    request: Json<Auth>,
    state: &State<Arc<DatabaseConnection>>,
) -> Result<Json<Fighter>, BadRequest<String>> {
    let device = Devices::find()
        .filter(Column::Fingerprint.eq(request.fingerprint.clone()))
        .one(state.as_ref())
        .await
        .map_err(|e| BadRequest(format!("Failed to find device: {}", e.to_string())))?;

    match device {
        Some(device) => {
            return Ok(Json(Fighter {
                fingerprint: device.fingerprint,
                fighter: device.fighter,
            }));
        }
        None => {
            let device = ActiveModel {
                fingerprint: ActiveValue::set(request.fingerprint.clone()),
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
                    Err(BadRequest("Failed to find device".to_string()))
                }
            }
        }
    }
}