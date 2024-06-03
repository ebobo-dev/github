use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
};

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
            Some(device) => match req.rocket().state::<crate::EboboState>() {
                Some(_) => match req.client_ip() {
                    Some(_) => request::Outcome::Success(Auth {
                        fingerprint: device.to_string(),
                    }),
                    None => {
                        request::Outcome::Error((Status::Unauthorized, AuthError::MissingAddress))
                    }
                },
                None => request::Outcome::Error((
                    Status::InternalServerError,
                    AuthError::InternalServerError("missing application state".to_string()),
                )),
            },
            None => request::Outcome::Error((Status::Unauthorized, AuthError::MissingFingerprint)),
        }
    }
}
