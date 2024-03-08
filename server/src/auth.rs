use crate::domain::device::Device;
use rocket::response::status::BadRequest;
use rocket::State;
use shuttle_persist::PersistInstance;

pub struct AuthState {
    pub persist: PersistInstance,
}

#[post("/", data = "<fingerprint>")]
pub fn auth(fingerprint: &str, state: &State<AuthState>) -> Result<String, BadRequest<String>> {
    if state
        .persist
        .list()
        .unwrap()
        .contains(&fingerprint.to_owned())
    {
        Ok("Welcome back!".to_owned())
    } else {
        let device = Device {
            fingerprint: fingerprint.to_owned(),
        };

        match state.persist.save(&fingerprint, &device) {
            Ok(_) => Ok("Welcome!".to_owned()),
            Err(e) => Err(BadRequest(e.to_string())),
        }
    }
}
