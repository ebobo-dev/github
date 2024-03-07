use shuttle_persist::PersistInstance;
use std::net::SocketAddr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Fingerprint {
    pub value: String,
    pub address: SocketAddr,
}

pub struct AuthState {
    pub persist: PersistInstance,
}