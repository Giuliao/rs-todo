use crate::diesel;
use diesel::prelude::*;

use crate::todo::structs::base::Base;
use crate::todo::ItemTypes;
use actix_web::body::BoxBody;
use actix_web::Responder;
use serde::Serialize;

use crate::database::establish_connection;
use crate::models::item::item::Item;
use crate::schema::to_do;
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
        let mut connection = establish_connection();
        let mut array_buffer = Vec::new();
        let items = to_do::table
            .order(to_do::columns::id.asc())
            .load::<Item>(&mut connection)
            .unwrap();

        for item in items {
            let status = TaskStatus::new(item.status.as_str());
            let item = todo_factory(&item.title, status);
            array_buffer.push(item);
        }

        TodoItems::new(array_buffer)
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
