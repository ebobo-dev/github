use ebobo_shared::{Auth, Device, Location, Utc};
use rocket::response::status::BadRequest;
use rocket::State;
use shuttle_persist::PersistInstance;
use rocket::serde::json::Json;

pub struct AuthState {
    pub persist: PersistInstance,
}

#[post("/authenticate", data = "<request>")]
pub fn authenticate(
    request: Json<Auth>,
    state: &State<AuthState>,
) -> Result<String, BadRequest<String>> {
    if state
        .persist
        .list()
        .unwrap()
        .contains(&request.fingerprint)
    {
        match state.persist.load::<Device>(&request.fingerprint) {
            Ok(mut d) => {
                if !d.is_active {
                    Err(BadRequest("Device is not active".to_owned()))?;
                }

                let greet: String;
                let mut name = d.fingerprint.clone();

                if d.locations.iter().all(|l| request.addr != l.address) {
                    d.locations.push(Location {
                        address: request.addr.to_owned(),
                        first_seen_at: Utc::now(),
                        last_seen_at: Utc::now(),
                        hits: 1,
                    });

                    greet = "You are in a new location!".to_owned();
                } else {
                    let location = d
                        .locations
                        .iter_mut()
                        .find(|l| l.address == request.addr)
                        .unwrap();

                    location.last_seen_at = Utc::now();
                    location.hits += 1;

                    if d.locations.iter().map(|l| l.hits).sum::<u32>() > 10 {
                        d.is_cat = true;
                    }

                    state.persist.save(&request.fingerprint, &d).unwrap();

                    greet = "You are in a known location!".to_owned();
                }

                if d.is_cat {
                    name = "🐱".to_owned();
                }

                Ok(format!("Welcome back, {}! {}", name, greet).to_owned())
            }
            Err(e) => Err(BadRequest(e.to_string())),
        }
    } else {
        let device = Device {
            fingerprint: request.fingerprint.to_owned(),
            is_cat: false,
            is_active: true,
            registered_at: Utc::now(),
            locations: vec![Location {
                address: request.addr.clone(),
                first_seen_at: Utc::now(),
                last_seen_at: Utc::now(),
                hits: 1,
            }],
        };

        match state.persist.save(&request.fingerprint, &device) {
            Ok(_) => Ok(format!("Welcome, {}!", device.fingerprint).to_owned()),
            Err(e) => Err(BadRequest(e.to_string())),
        }
    }
}
