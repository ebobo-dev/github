#[get("/")]
pub fn index () -> &'static str {

    // let fingerprints = state
    // .persist
    // .list()
    // .unwrap()
    // .iter()
    // .map(|f| state.persist.load::<Fingerprint>(f).unwrap())
    // .collect();

    "shuttle ebobo"
}