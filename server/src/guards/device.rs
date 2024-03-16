use std::net::IpAddr;

use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
};

pub struct Device
{
    pub fingerprint: String,
    pub addr: Option<IpAddr>,
}

#[derive(Debug)]
pub enum FingerprintError {
    Unauthorized,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Device {
    type Error = FingerprintError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.headers().get_one(ebobo_shared::AUTH_HEADER) {
            Some(device) => request::Outcome::Success(Device {
                fingerprint: device.to_string(),
                addr: req.client_ip(),
        }),
            None => request::Outcome::Error((Status::Unauthorized, FingerprintError::Unauthorized)),
        }
    }
}
