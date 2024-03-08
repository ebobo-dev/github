use std::net::SocketAddr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub fingerprint: String,
    pub address: SocketAddr,
}