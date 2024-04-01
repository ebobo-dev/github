pub use chrono::{DateTime, Utc, Duration, NaiveDateTime};
pub use serde::{Deserialize, Serialize};

pub const AUTH_HEADER: &str = "EBOBO-FINGERPRINT";

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    pub greet: String,
    pub fighters: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice(String);