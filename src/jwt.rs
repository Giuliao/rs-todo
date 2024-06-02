use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::future::{ok, Ready};
pub struct JwtToken {
    pub message: String,
}

impl FromRequest for JwtToken {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.headers().get("token") {
            Some(data) => {
                let token = JwtToken {
                    message: data.to_str().unwrap().to_string(),
                };
                ok(token)
            }
            None => {
                let token = JwtToken {
                    message: String::from("Nothing found"),
                };
                ok(token)
            }
        }
    }
}
