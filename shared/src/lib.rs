pub use chrono::{DateTime, Utc, Duration, NaiveDateTime};
pub use serde::{Deserialize, Serialize};

pub const AUTH_HEADER: &str = "EBOBO-FINGERPRINT";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub fighter: bool,
    pub greet: String,
    pub fighters: Vec<Fighter>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fighter(String);

impl Fighter {
    pub fn new(name: &str) -> Self {
        Self(name.to_owned())
    }

    pub fn name(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice(String);

impl Choice {
    pub fn new(name: &str) -> Self {
        Self(name.to_owned())
    }

    pub fn fighter(&self) -> String {
        self.0.clone()
    }
}