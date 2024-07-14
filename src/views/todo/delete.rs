use crate::diesel;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::json_serialization::todo_item::TodoItem;
use crate::json_serialization::todo_items::TodoItems;
use crate::jwt::JwtToken;
use crate::models::item::item::Item;
use crate::schema::to_do;

pub async fn delete(todo_item: web::Json<TodoItem>, token: JwtToken) -> HttpResponse {
    let mut connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(&todo_item.title.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();
    let _ = diesel::delete(&items[0]).execute(&mut connection);
    HttpResponse::Ok().json(TodoItems::get_state())
}
