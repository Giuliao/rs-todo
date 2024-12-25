use crate::diesel;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

use crate::database::{establish_connection, DB};
use crate::json_serialization::login::Login;
use crate::json_serialization::login_response::LoginResponse;
use crate::jwt::JwtToken;
use crate::models::user::user::User;
use crate::schema::users;

pub async fn login(credentials: web::Json<Login>, _: DB) -> HttpResponse {
    let password = credentials.password.clone();
    let users = users::table
        .filter(users::columns::username.eq(credentials.username.clone()))
        .load::<User>(&mut establish_connection())
        .unwrap();

    if users.is_empty() {
        return HttpResponse::NotFound().await.unwrap();
    } else if users.len() > 1 {
        return HttpResponse::Conflict().await.unwrap();
    }

    match users[0].verify(password) {
        true => {
            let token = JwtToken::new(users[0].id);
            let raw_token = token.encode();
            let response = LoginResponse {
                token: raw_token.clone(),
            };
            let body = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok()
                .append_header(("token", raw_token))
                .json(&body)
        }
        false => HttpResponse::Unauthorized().finish(),
    }
}
