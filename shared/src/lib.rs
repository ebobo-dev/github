pub use chrono::{DateTime, Utc, Duration, NaiveDateTime};
pub use serde::{Deserialize, Serialize};

pub const AUTH_HEADER: &str = "EBOBO-FINGERPRINT";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub greet: String,
    pub fighters: Vec<Fighter>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fighter(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice(pub String);