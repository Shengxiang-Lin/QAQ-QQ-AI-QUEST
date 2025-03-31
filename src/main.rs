use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use QAQ::config::config;
use QAQ::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    QAQ::initialize_database_manager().await;
    HttpServer::new(|| {
        App::new()
        .configure(routes::config)
    })
    .bind(format!("127.0.0.1:{}", config::PORT))?
    .run()
    .await
}