mod routes;
mod handlers;

use actix_web::{web, App, HttpServer};
use QAQ::config::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .configure(routes::config)
    })
    .bind(format!("127.0.0.1:{}", config::PORT))?
    .run()
    .await
}