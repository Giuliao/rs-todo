use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};

use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtToken {
    pub user_id: i32,
    #[serde(with = "ts_seconds")]
    pub minted: DateTime<Utc>,
    pub exp: usize,
}

impl JwtToken {
    pub fn new(user_id: i32) -> Self {
        let timestamp = Utc::now();
        let config = Config::new();
        let minutes = config
            .map
            .get("EXPIRATION_MINUTES")
            .unwrap()
            .as_i64()
            .unwrap();
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::minutes(minutes))
            .expect("valid timestamp")
            .timestamp() as usize;

        JwtToken {
            user_id,
            minted: timestamp,
            exp: expiration,
        }
    }

    pub fn get_key() -> String {
        let config = Config::new();
        let key_str = config.map.get("SECRET_KEY").unwrap().as_str().unwrap();
        key_str.to_owned()
    }

    pub fn encode(self) -> String {
        let key = EncodingKey::from_secret(JwtToken::get_key().as_ref());
        encode(&Header::default(), &self, &key).unwrap()
    }
    pub fn from_token(token: String) -> Result<Self, String> {
        let key = DecodingKey::from_secret(JwtToken::get_key().as_ref());
        let token_result = decode::<JwtToken>(&token, &key, &Validation::new(Algorithm::HS256));

        match token_result {
            Ok(data) => Ok(data.claims),
            Err(error) => {
                let message = format!("{}", error);
                Err(message)
            }
        }
    }
}

impl FromRequest for JwtToken {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.headers().get("token") {
            Some(data) => {
                let raw_token = data.to_str().unwrap().to_string();
                let token_result = JwtToken::from_token(raw_token);
                match token_result {
                    Ok(token) => ok(token),
                    Err(message) => {
                        if message == "ExpiredSignature" {
                            return err(ErrorUnauthorized("token has expired"));
                        }

                        let error = ErrorUnauthorized("token can't be decoded");
                        err(error)
                    }
                }
            }
            None => {
                let error = ErrorUnauthorized("token not in header under key 'token'");
                err(error)
            }
        }
    }
}
