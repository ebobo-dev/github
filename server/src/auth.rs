use crate::*;
use rocket::response::status::BadRequest;
use std::net::SocketAddr;

#[post("/", data = "<fingerprint>")]
pub fn auth(
    fingerprint: &str,
    state: &State<AuthState>,
    remote_addr: SocketAddr,
) -> Result<Json<Vec<Fingerprint>>, BadRequest<String>> {
    let fingerprint = Fingerprint {
        value: fingerprint.to_string(),
        address: remote_addr
    };

    let _ = state
        .persist
        .save::<Fingerprint>(
            format!("fingerprint_{}", &fingerprint.value.as_str()).as_str(),
            fingerprint.clone(),
        )
        .map_err(|e| BadRequest(Some(e.to_string())));

    let fingerprints = state
        .persist
        .list()
        .unwrap()
        .iter()
        .map(|f| state.persist.load::<Fingerprint>(f).unwrap())
        .collect();

    Ok(Json(fingerprints))
}
