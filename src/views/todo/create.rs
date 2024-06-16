use actix_web::{HttpRequest, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;

use crate::json_serialization::todo_items::TodoItems;
use crate::processes::process_input;
use crate::state::read_file;
use crate::todo::{enums::TaskStatus, todo_factory};

pub async fn create(req: HttpRequest) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");
    let title: String = req.match_info().get("title").unwrap().to_string();

    let item = todo_factory(&title.as_str(), TaskStatus::PENDING);

    process_input(item, "create".to_string(), &state);

    HttpResponse::Ok().json(TodoItems::get_state())
}
