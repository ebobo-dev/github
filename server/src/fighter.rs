use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::entities::{prelude::*, fighters};

pub struct Fighter {
    pub fingerprint: String,
    pub fighter: String,
    pub rank: i32,
}

#[derive(Debug)]
pub enum FighterError {
    MissingFingerprint,
    MissingFighter,
    InternalServerError(String),
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Fighter {
    type Error = FighterError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.headers().get_one(ebobo_shared::AUTH_HEADER) {
            Some(device) => match req.rocket().state::<crate::EboboState>() {
                Some(state) => {
                    let user = Fighters::find()
                        .filter(fighters::Column::Fingerprint.eq(device))
                        .one(state.db.as_ref())
                        .await;
                    match user {
                        Ok(Some(u)) => request::Outcome::Success(Fighter {
                            fighter: u.emo,
                            fingerprint: u.fingerprint,
                            rank: u.rank,
                        }),
                        Ok(None) => request::Outcome::Error((
                            Status::Unauthorized,
                            FighterError::MissingFighter,
                        )),
                        Err(e) => request::Outcome::Error((
                            Status::InternalServerError,
                            FighterError::InternalServerError(e.to_string()),
                        )),
                    }
                }
                None => request::Outcome::Error((
                    Status::InternalServerError,
                    FighterError::InternalServerError("missing application state".to_string()),
                )),
            },
            None => {
                request::Outcome::Error((Status::Unauthorized, FighterError::MissingFingerprint))
            }
        }
    }
}
