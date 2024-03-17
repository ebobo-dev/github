use crate::entities::prelude::*;
use crate::guards::auth::Auth;
use ebobo_shared::*;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::*;
use std::sync::Arc;

#[options("/authenticate")]
pub async fn options() {}

#[post("/authenticate")]
pub async fn authenticate(
    auth: Auth,
    state: &State<Arc<DatabaseConnection>>,
) -> Result<Json<Fighter>, BadRequest<String>> {
    let location_id = get_location_id(&auth, state).await;
    let device_id = get_device_id(&auth, state).await?;

    if let Some(id) = location_id {
        let device_location = DevicesLocations::find()
            .filter(crate::entities::devices_locations::Column::DeviceId.eq(device_id))
            .filter(crate::entities::devices_locations::Column::LocationId.eq(id))
            .one(state.as_ref())
            .await
            .map_err(|e| {
                BadRequest(format!("Failed to find device location: {}", e.to_string()))
            })?;

        if device_location == None {
            let device_location = crate::entities::devices_locations::ActiveModel {
                device_id: ActiveValue::set(device_id),
                location_id: ActiveValue::set(id),
                ..Default::default()
            };

            DevicesLocations::insert(device_location)
                .exec(state.as_ref())
                .await
                .map_err(|e| {
                    BadRequest(format!(
                        "Failed to insert device location: {}",
                        e.to_string()
                    ))
                })?;
        }
    }

    let device = Devices::find()
        .filter(crate::entities::devices::Column::Id.eq(device_id))
        .one(state.as_ref())
        .await
        .map_err(|e| BadRequest(format!("Failed to find device: {}", e.to_string())))?
        .ok_or(BadRequest("Failed to find device".to_string()))?;

    Ok(Json(Fighter {
        fingerprint: device.fingerprint,
        fighter: device.fighter,
    }))
}

async fn get_location_id(auth: &Auth, state: &State<Arc<DatabaseConnection>>) -> Option<i32> {
    if let Some(addr) = auth.addr {
        let location = Locations::find()
            .filter(crate::entities::locations::Column::Address.eq(addr.to_string()))
            .one(state.as_ref())
            .await
            .map_err(|e| BadRequest(format!("Failed to find location: {}", e.to_string())))
            .ok()?;

        match location {
            Some(l) => Some(l.id),
            None => {
                let location = crate::entities::locations::ActiveModel {
                    address: ActiveValue::set(addr.to_string()),
                    ..Default::default()
                };

                let result: InsertResult<crate::entities::locations::ActiveModel> =
                    Locations::insert(location)
                        .exec(state.as_ref())
                        .await
                        .map_err(|e| {
                            BadRequest(format!("Failed to insert location: {}", e.to_string()))
                        })
                        .ok()?;

                Some(result.last_insert_id)
            }
        };
    }
    None
}

async fn get_device_id(
    auth: &Auth,
    state: &State<Arc<DatabaseConnection>>,
) -> Result<i32, BadRequest<String>> {
    let device = Devices::find()
        .filter(crate::entities::devices::Column::Fingerprint.eq(auth.fingerprint.clone()))
        .one(state.as_ref())
        .await
        .map_err(|e| BadRequest(format!("Failed to find device: {}", e.to_string())))?;

    match device {
        Some(d) => Ok(d.id),
        None => {
            let device = crate::entities::devices::ActiveModel {
                fingerprint: ActiveValue::set(auth.fingerprint.clone()),
                ..Default::default()
            };

            let result: InsertResult<crate::entities::devices::ActiveModel> =
                Devices::insert(device)
                    .exec(state.as_ref())
                    .await
                    .map_err(|e| {
                        BadRequest(format!("Failed to insert device: {}", e.to_string()))
                    })?;

            Ok(result.last_insert_id)
        }
    }
}
