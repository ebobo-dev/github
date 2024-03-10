use ebobo_shared::*;

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub fingerprint: String,
    pub hits: u32,
    pub is_active: bool,
    pub is_cat: bool,
    pub registered_at: DateTime<Utc>,
    pub locations: Vec<Location>,
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub address: String,
    pub is_home: bool,
    pub first_seen_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub hits: u32,
}