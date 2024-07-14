use crate::diesel;
use crate::diesel::prelude::*;
use actix_web::{web, HttpResponse};

use crate::database::establish_connection;
use crate::json_serialization::todo_item::TodoItem;
use crate::json_serialization::todo_items::TodoItems;

use crate::jwt::JwtToken;
use crate::schema::to_do;

pub async fn edit(todo_item: web::Json<TodoItem>, token: JwtToken) -> HttpResponse {
    let mut connection = establish_connection();
    let results = to_do::table.filter(to_do::columns::title.eq(&todo_item.title));
    let _ = diesel::update(results)
        .set(to_do::columns::status.eq("DONE"))
        .execute(&mut connection);

    HttpResponse::Ok().json(TodoItems::get_state())
}
