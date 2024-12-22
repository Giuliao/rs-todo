#[macro_use]
extern crate diesel;
extern crate dotenvy;

mod config;
mod database;
mod json_serialization;
mod jwt;
mod models;
mod schema;
mod todo;
mod views;

use actix_service::Service;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                println!("{:?}", req);
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
