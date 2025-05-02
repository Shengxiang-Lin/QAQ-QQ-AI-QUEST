use actix_web::{App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use QAQ::config::get_config;
use QAQ::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    QAQ::initialize_database_manager().await;
    HttpServer::new(|| {
        App::new()
        .wrap(
            Cors::default()
                .allow_any_origin() // 允许所有来源
                .allow_any_method() // 允许所有 HTTP 方法
                .allow_any_header() // 允许所有请求头
                .max_age(3600),     // 设置预检请求的缓存时间
        )
        .configure(routes::config)
    })
    .bind(format!("127.0.0.1:{}", get_config().rust_port))?
    .run()
    .await
}