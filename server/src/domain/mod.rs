use ebobo_shared::*;

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub id: i32,
    pub fingerprint: String,
    pub fighter: Option<char>,
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub id: i32,
    pub address: String
}
