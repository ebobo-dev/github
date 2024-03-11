use crate::domain::Device;
use libsql::params;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;

#[get("/admin")]
pub fn index(conn: &State<crate::Database>) -> Result<Json<Vec<Device>>, BadRequest<String>> {
    let devices = conn.turso.query("SELECT * FROM Devices", params![])
        .persist
        .list()
        .unwrap()
        .iter()
        .map(|f| state.persist.load::<Device>(f).unwrap())
        .collect();
    Ok(Json(devices))
}

#[post("/admin/reset")]
pub fn reset(conn: &State<crate::Database>) -> Result<(), BadRequest<String>> {
    conn.turso.query("DELETE FROM Devices", params![])
        .persist
        .reset()
        .unwrap();
    Ok(())
}
