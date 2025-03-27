use actix_web::{web, App, HttpServer};
use crate::routes

static const PORT: usize = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .configure(routes::config)
    })
    .bind("127.0.0.1:{}",PORT)?
    .run()
    .await
}