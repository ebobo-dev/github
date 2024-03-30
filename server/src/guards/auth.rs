use std::net::IpAddr;

use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
};
use sea_orm::{prelude::*, *};

use ebobo_shared::*;

use crate::{entities::prelude::*, entities::requests::*, AppState};

pub struct Auth {
    pub fingerprint: String,
    pub addr: Option<IpAddr>,
}

#[derive(Debug)]
pub enum FingerprintError {
    MissingFingerprint,
    InaccessibleDatabase,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = FingerprintError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.headers().get_one(ebobo_shared::AUTH_HEADER) {
            Some(device) => match req.rocket().state::<AppState>() {
                Some(state) => {
                    Requests::insert(ActiveModel {
                        id: Default::default(),
                        fingerprint: ActiveValue::set(device.to_string()),
                        address: ActiveValue::set(match req.client_ip() {
                            Some(addr) => Some(addr.to_string()),
                            None => None,
                        }),
                        timestamp: ActiveValue::set(Utc::now().naive_utc()),
                    })
                    .exec(state.db.as_ref())
                    .await
                    .unwrap(); // TODO: Handle error

                    request::Outcome::Success(Auth {
                        fingerprint: device.to_string(),
                        addr: req.client_ip(),
                    })
                }
                None => request::Outcome::Error((
                    Status::InternalServerError,
                    FingerprintError::InaccessibleDatabase,
                )),
            },
            None => request::Outcome::Error((
                Status::Unauthorized,
                FingerprintError::MissingFingerprint,
            )),
        }
    }
}
