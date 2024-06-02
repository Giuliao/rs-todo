use crate::json_serialization::todo_item::TodoItem;
use crate::json_serialization::todo_items::TodoItems;
use crate::processes::process_input;
use crate::todo::enums::TaskStatus;
use crate::todo::todo_factory;

use actix_web::{web, HttpResponse};

use crate::state::read_file;
use serde_json::value::Value;
use serde_json::Map;

pub async fn edit(todo_item: web::Json<TodoItem>) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");
    let status: TaskStatus;
    match &state.get(&todo_item.title) {
        Some(result) => status = TaskStatus::new(result.as_str().unwrap()),
        None => {
            return HttpResponse::NotFound().json(format!("{} not in state", &todo_item.title));
        }
    }

    let existing_item = todo_factory(todo_item.title.as_str(), status.clone());
    if &status.stringify() == todo_item.status.as_str() {
        return HttpResponse::Ok().json(TodoItems::get_state());
    }

    process_input(existing_item, "edit".to_owned(), &state);

    HttpResponse::Ok().json(TodoItems::get_state())
}
