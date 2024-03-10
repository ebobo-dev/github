use crate::domain::*;
use ebobo_shared::*;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use shuttle_persist::PersistInstance;

pub struct AuthState {
    pub persist: PersistInstance,
}

#[post("/authenticate", data = "<request>")]
pub fn authenticate(
    request: Json<Auth>,
    state: &State<AuthState>,
) -> Result<String, BadRequest<String>> {
    if state.persist.list().unwrap().contains(&request.fingerprint) {
        match state.persist.load::<Device>(&request.fingerprint) {
            Ok(mut d) => {
                if !d.is_active {
                    d.hits += 1;

                    if d.hits > 10 {
                        d.is_active = true;
                        d.hits = 0;
                    }

                    state.persist.save(&request.fingerprint, &d).unwrap();

                    Err(BadRequest("Device is not active".to_owned()))?;
                }

                let mut msg = "Welcome back, ".to_owned();

                msg += match d.is_cat {
                    true => "🐱",
                    false => d.fingerprint.as_str(),
                };

                msg += ".";

                if Utc::now() - d.locations.iter().map(|l| l.last_seen_at).max().unwrap()
                    > Duration::try_minutes(1).unwrap()
                {
                    msg += " Long time no see.";

                    if Utc::now() - d.locations.iter().map(|l| l.last_seen_at).max().unwrap()
                        > Duration::try_minutes(5).unwrap()
                        && !d.is_cat
                    {
                        d.is_active = false;
                        msg += " Deactivating for 5 hits."
                    }
                }

                if d.locations.iter().all(|l| request.addr != l.address) {
                    d.locations.push(Location {
                        address: request.addr.to_owned(),
                        is_home: false,
                        first_seen_at: Utc::now(),
                        last_seen_at: Utc::now(),
                        hits: 1,
                    });

                    msg += " You are in a new location.";
                } else {
                    if d.locations.iter().map(|l| l.hits).sum::<u32>() > 10 {
                        d.is_cat = true;
                    }

                    let location = d
                        .locations
                        .iter_mut()
                        .find(|l| l.address == request.addr)
                        .unwrap();

                    location.last_seen_at = Utc::now();
                    location.hits += 1;

                    if location.hits > 20 {
                        location.is_home = true;
                    }

                    msg += " You are in a known location.";

                    if location.is_home {
                        msg += " Welcome home.";
                    }
                }

                state.persist.save(&request.fingerprint, &d).unwrap();

                Ok(msg)
            }
            Err(e) => Err(BadRequest(e.to_string())),
        }
    } else {
        let device = Device {
            fingerprint: request.fingerprint.to_owned(),
            hits: 0,
            is_cat: false,
            is_active: true,
            registered_at: Utc::now(),
            locations: vec![Location {
                address: request.addr.clone(),
                is_home: false,
                first_seen_at: Utc::now(),
                last_seen_at: Utc::now(),
                hits: 1,
            }],
        };

        match state.persist.save(&request.fingerprint, &device) {
            Ok(_) => Ok(format!("Welcome, {}. Nice to meet you.", device.fingerprint).to_owned()),
            Err(e) => Err(BadRequest(e.to_string())),
        }
    }
}
