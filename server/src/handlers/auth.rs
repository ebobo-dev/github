use crate::domain::device::Device;
use crate::domain::location::Location;
use chrono::Utc;
use rocket::response::status::BadRequest;
use rocket::State;
use shuttle_persist::PersistInstance;
use std::net::SocketAddr;

pub struct AuthState {
    pub persist: PersistInstance,
}

#[post("/", data = "<fingerprint>")]
pub fn authenticate(
    fingerprint: &str,
    state: &State<AuthState>,
    remote_addr: SocketAddr,
) -> Result<String, BadRequest<String>> {
    if state
        .persist
        .list()
        .unwrap()
        .contains(&fingerprint.to_owned())
    {
        match state.persist.load::<Device>(&fingerprint) {
            Ok(mut d) => {
                if !d.is_active {
                    Err(BadRequest("Device is not active".to_owned()))?;
                }

                if d.locations.iter().all(|l| l.address != remote_addr) {
                    d.locations.push(Location {
                        address: remote_addr,
                        first_seen_at: Utc::now(),
                        last_seen_at: Utc::now(),
                        hits: 1,
                    });
                } else {
                    let location = d
                        .locations
                        .iter_mut()
                        .find(|l| l.address == remote_addr)
                        .unwrap();

                    location.last_seen_at = Utc::now();
                    location.hits += 1;

                    if d.locations.iter().map(|l| l.hits).sum::<u32>() > 10 {
                        d.is_cat = true;
                    }

                    state.persist.save(&fingerprint, &d).unwrap();
                }

                let name = if d.is_cat { "🐱" } else { fingerprint };

                Ok(format!("Welcome back, {}!", name).to_owned())
            }
            Err(e) => Err(BadRequest(e.to_string())),
        }
    } else {
        let device = Device {
            fingerprint: fingerprint.to_owned(),
            is_cat: false,
            is_active: true,
            registered_at: Utc::now(),
            locations: vec![Location {
                address: remote_addr,
                first_seen_at: Utc::now(),
                last_seen_at: Utc::now(),
                hits: 1,
            }],
        };

        match state.persist.save(&fingerprint, &device) {
            Ok(_) => Ok(format!("Welcome, {}!", device.fingerprint).to_owned()),
            Err(e) => Err(BadRequest(e.to_string())),
        }
    }
}
