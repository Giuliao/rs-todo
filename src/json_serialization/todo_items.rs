use crate::todo::structs::base::Base;
use crate::todo::ItemTypes;
use actix_web::body::BoxBody;
use actix_web::Responder;
use serde::Serialize;
use serde_json::value::Value;
use serde_json::Map;

use crate::state::read_file;
use crate::todo::enums::TaskStatus;
use crate::todo::todo_factory;

#[derive(Serialize)]
pub struct TodoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl TodoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> Self {
        let mut pending_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Pending(item) => pending_array_buffer.push(item.super_struct),
                ItemTypes::Done(item) => done_array_buffer.push(item.super_struct),
            }
        }

        let pending_item_count = pending_array_buffer.len() as i8;
        let done_count = done_array_buffer.len() as i8;
        TodoItems {
            pending_items: pending_array_buffer,
            done_items: done_array_buffer,
            pending_item_count,
            done_item_count: done_count,
        }
    }

    pub fn get_state() -> Self {
        let state: Map<String, Value> = read_file("./state.json");
        let mut array_buf = Vec::new();
        for (k, v) in state {
            let status = TaskStatus::from_string(v.as_str().unwrap().to_string());
            let item: ItemTypes = todo_factory(&k, status);
            array_buf.push(item);
        }

        TodoItems::new(array_buf)
    }
}

impl Responder for TodoItems {
    type Body = BoxBody;
    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        actix_web::HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}
