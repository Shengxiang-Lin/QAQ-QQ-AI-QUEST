use actix_web::{post, web, HttpResponse, Responder};
use crate::{ll_one_bot::interface::*, pipeline::handle_message_pipeline, QQ_SENDER};

#[post("/")]
pub async fn show_info(
    info: Result<web::Json<LLOneBot>, actix_web::Error>, // 使用 Result 包装解析结果
) -> impl Responder {
    match info {
        Ok(valid_info) => {
            println!("Received info: {:?}", valid_info);
            let sendback = handle_message_pipeline(valid_info.into_inner()).await.unwrap();
            QQ_SENDER.send_qq_post(&sendback).await.unwrap();
            return HttpResponse::Ok().body("Success");
             // 返回成功响应
        }
        Err(err) => {
            println!("Failed to parse LLOneBotPrivate: {:?}", err); // 打印错误信息
            return HttpResponse::BadRequest().body(format!("Invalid request body: {}", err)) // 返回 400 错误
        }
    };
    
}

