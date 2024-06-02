mod json_serialization;
mod jwt;
mod processes;
mod state;
mod todo;
mod views;

use actix_service::Service;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, srv| {
                println!("{:?}", req);
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory);
        return app;
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
