use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
};
use sea_orm::{prelude::*, *};

use ebobo_shared::*;

use crate::{entities::prelude::*, entities::requests::*, AppState};

pub struct Auth {
    pub fingerprint: String,
}

#[derive(Debug)]
pub enum AuthError {
    MissingFingerprint,
    MissingAddress,
    InternalServerError(String),
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.headers().get_one(ebobo_shared::AUTH_HEADER) {
            Some(device) => match req.rocket().state::<AppState>() {
                Some(state) => {
                    match req.client_ip() {
                        Some(addr) => match Requests::insert(ActiveModel {
                            id: ActiveValue::set(Uuid::new_v4()),
                            fingerprint: ActiveValue::set(device.to_string()),
                            address: ActiveValue::set(Some(addr.to_string())),
                            timestamp: ActiveValue::set(Utc::now().naive_utc()),
                        })
                        .exec(state.db.as_ref())
                        .await
                        {
                            Ok(_) => request::Outcome::Success(Auth {
                                fingerprint: device.to_string(),
                            }),
                            Err(e) => request::Outcome::Error((
                                Status::InternalServerError,
                                AuthError::InternalServerError(e.to_string()),
                            )),
                        },
                        None => request::Outcome::Error((Status::Unauthorized, AuthError::MissingAddress)),
                    }
                }
                None => request::Outcome::Error((
                    Status::InternalServerError,
                    AuthError::InternalServerError("Missing application state".to_string()),
                )),
            },
            None => request::Outcome::Error((Status::Unauthorized, AuthError::MissingFingerprint)),
        }
    }
}
