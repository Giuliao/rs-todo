use crate::json_serialization::todo_items::TodoItems;
use actix_web::Responder;

pub async fn get() -> impl Responder {
    TodoItems::get_state()
}
