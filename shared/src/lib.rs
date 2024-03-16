pub use chrono::{DateTime, Utc, Duration};
pub use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Fighter {
    pub fingerprint: String,
    pub fighter: Option<String>,
}