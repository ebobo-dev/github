use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
};

use crate:: AppState;

pub struct Fighter {
    pub fingerprint: String,
    pub fighter: String,
    pub rank: i32,
    pub count: i32,
    pub kd: f32,
}

#[derive(Debug)]
pub enum FighterError {
    MissingFighter,
    InternalServerError(String),
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Fighter {
    type Error = FighterError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        unimplemented!()
    }
}
