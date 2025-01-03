use crate::diesel;
use crate::diesel::prelude::*;
use actix_web::{web, HttpResponse};

use crate::json_serialization::todo_item::TodoItem;
use crate::json_serialization::todo_items::TodoItems;

use crate::database::DB;
use crate::jwt::JwtToken;
use crate::schema::to_do;

pub async fn edit(todo_item: web::Json<TodoItem>, token: JwtToken, mut db: DB) -> HttpResponse {
    let results = to_do::table
        .filter(to_do::columns::title.eq(&todo_item.title))
        .filter(to_do::columns::user_id.eq(token.user_id));

    let _ = diesel::update(results)
        .set(to_do::columns::status.eq("DONE"))
        .execute(&mut db.connection);

    HttpResponse::Ok().json(TodoItems::get_state(token.user_id))
}
