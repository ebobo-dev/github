use std::net::IpAddr;

use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
};

pub struct Auth {
    pub fingerprint: String,
    pub addr: Option<IpAddr>,
}

#[derive(Debug)]
pub enum FingerprintError {
    Unauthorized,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = FingerprintError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let fingerprint = req.headers().get_one(ebobo_shared::AUTH_HEADER);
        match fingerprint {
            Some(device) => request::Outcome::Success(Auth {
                fingerprint: device.to_string(),
                addr: req.client_ip(),
            }),
            None => request::Outcome::Error((Status::Unauthorized, FingerprintError::Unauthorized)),
        }
    }
}
