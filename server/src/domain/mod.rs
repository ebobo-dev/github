use ebobo_shared::*;

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub fingerprint: String,
    pub fighter: Option<char>,
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub address: String
}
