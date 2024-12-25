use crate::{json_serialization::todo_items::TodoItems, jwt::JwtToken};
use actix_web::Responder;

pub async fn get(token: JwtToken) -> impl Responder {
    TodoItems::get_state(token.user_id)
}
