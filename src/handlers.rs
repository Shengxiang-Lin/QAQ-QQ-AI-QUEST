use actix_web::{get,post, web, HttpResponse, Responder};
use crate::ll_one_bot::interface::*;
use crate::services::CLIENT_MANAGER;

#[post("/")]
pub async fn show_info(
    info: Result<web::Json<LLOneBotMessage>, actix_web::Error>, // 使用 Result 包装解析结果
) -> impl Responder {
    match info {
        Ok(valid_info) => {
            println!("123");
            println!("Received info: {:?}", valid_info);
            HttpResponse::Ok().json(valid_info) // 返回成功响应
        }
        Err(err) => {
            println!("Failed to parse LLOneBotMessage: {:?}", err); // 打印错误信息
            HttpResponse::BadRequest().body(format!("Invalid request body: {}", err)) // 返回 400 错误
        }
    }
}

