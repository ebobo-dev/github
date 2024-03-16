pub use chrono::{DateTime, Utc, Duration};
pub use serde::{Deserialize, Serialize};

pub const AUTH_HEADER: &str = "EBOBO_FINGERPRINT";

#[derive(Serialize, Deserialize)]
pub struct Fighter {
    pub fingerprint: String,
    pub fighter: Option<String>,
}