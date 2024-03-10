use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{HeaderMap, Status};
use rocket::{Request, Response, Data};

pub struct HeaderInspectFairing;

impl Fairing for HeaderInspectFairing {
    fn info(&self) -> Info {
        Info {
            name: "Header Inspect Fairing",
            kind: Kind::Request | Kind::Response,
        }
    }

    fn on_request<'a>(&self, request: &mut Request<'a>, _: &Data) {
        let headers = request.headers();
        // Inspect and process the headers as needed
        for (name, value) in headers.iter() {
            println!("Header: {} = {}", name, value.to_str().unwrap());
        }
    }

    fn on_response(&self, _: &Request<'_>, response: &mut Response<'_>) {
        // Modify the response as needed
        response.set_status(Status::Ok);
    }
}