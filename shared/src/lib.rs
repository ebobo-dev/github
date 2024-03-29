pub use chrono::{DateTime, Utc, Duration, NaiveDateTime};
pub use serde::{Deserialize, Serialize};

pub const AUTH_HEADER: &str = "EBOBO-FINGERPRINT";

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Fighter {
    pub fingerprint: String,
    pub fighter: Option<String>,
}