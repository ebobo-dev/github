use super::location::Location;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub fingerprint: String,
    pub is_cat: bool,
    pub is_active: bool,
    pub registered_at: DateTime<Utc>,
    pub locations: Vec<Location>,
}
