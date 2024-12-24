use crate::diesel;
use actix_web::{HttpRequest, HttpResponse};
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::json_serialization::todo_items::TodoItems;
use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;
use crate::schema::to_do;

pub async fn create(req: HttpRequest) -> HttpResponse {
    let title: String = req.match_info().get("title").unwrap().to_string();
    let mut connection = establish_connection();

    let items = to_do::table
        .filter(to_do::columns::title.eq(&title.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();

    if items.is_empty() {
        let new_post = NewItem::new(title, 1);
        let _ = diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&mut connection);
    }

    HttpResponse::Ok().json(TodoItems::get_state())
}
