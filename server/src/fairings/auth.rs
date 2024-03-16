use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{request, response, Request, Response};

pub struct Auth;

#[rocket::async_trait]
impl Fairing for Auth {
    fn info(&self) -> Info {
        Info {
            name: "Check Auth header on request",
            kind: Kind::Request,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        match request.headers().get_one("EBOBO-AUTH") {
            Some(token) => {
                println!("Token: {}", token);
            }
            None => {
                match request
                    .uri()
                    .path()
                    .as_str()
                    .starts_with("/admin")
                {
                    true => {
                        
                    }
                    false => {
                        response.set_status(rocket::http::Status::Unauthorized);
                    }
                }
            }
        }
    }
}
