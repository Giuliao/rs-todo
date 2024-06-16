use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;

use crate::json_serialization::todo_item::TodoItem;
use crate::json_serialization::todo_items::TodoItems;
use crate::jwt::JwtToken;
use crate::processes::process_input;
use crate::state::read_file;
use crate::todo::enums::TaskStatus;
use crate::todo::todo_factory;

pub async fn delete(todo_item: web::Json<TodoItem>, token: JwtToken) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");
    let status: TaskStatus;

    match &state.get(&todo_item.title) {
        Some(result) => {
            status = TaskStatus::from_string(result.as_str().unwrap().to_string());
        }
        _ => {
            return HttpResponse::NotFound().json(format!("{} not in state", &todo_item.title));
        }
    }

    let existing_item = todo_factory(todo_item.title.as_str(), status.clone());
    process_input(existing_item, "delete".to_owned(), &state);
    HttpResponse::Ok().json(TodoItems::get_state())
}
