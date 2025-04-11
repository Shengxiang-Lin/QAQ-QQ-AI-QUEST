/*use actix_web::{post, web, HttpResponse, Responder};
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
use actix_web::HttpRequest;
use actix_web::FromRequest;
use crate::handlers::web::Bytes;
use std::io::Bytes as OtherBytes;
use std::str::Bytes as OtherOtherBytes;
#[post("/")]
pub async fn show_info(
    payload: web::Payload,
    req: HttpRequest,
) -> impl Responder {
    // 1. 首先将整个payload读取为Bytes
    let body = web::Bytes::from_request(&req, &mut payload.into_inner())
        .await
        .unwrap_or_else(|_| Bytes::new());
    
    // 2. 输出原始请求体（调试用）
    println!("Raw request body: {:?}", body);
    
    // 3. 尝试解析为JSON
    match web::Json::<LLOneBot>::from_request(&req, &mut body.clone().into()).await {
        Ok(valid_info) => {
            println!("Parsed info: {:?}", valid_info);
            let sendback = handle_message_pipeline(valid_info.into_inner()).await.unwrap();
            QQ_SENDER.send_qq_post(&sendback).await.unwrap();
            HttpResponse::Ok().body("Success")
        }
        Err(err) => {
            println!("Failed to parse. Raw body was: {:?}", String::from_utf8_lossy(&body));
            HttpResponse::BadRequest().body(format!("Invalid request body: {}", err))
        }
    }
}*/
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use crate::{ll_one_bot::interface::*, pipeline::handle_message_pipeline, QQ_SENDER};
use actix_web::FromRequest;

#[post("/")]
pub async fn show_info(
    payload: web::Payload,
    req: HttpRequest,
) -> impl Responder {
    // 1. 获取原始请求体
    let body = match web::Bytes::from_request(&req, &mut payload.into_inner()).await {
        Ok(b) => b,
        Err(e) => {
            eprintln!("❌ Failed to read request body: {:?}", e);
            return HttpResponse::BadRequest().body("Invalid request body");
        }
    };

    // 2. 打印原始请求内容（使用克隆体）
    let body_clone = body.clone();
    let body_str = String::from_utf8_lossy(&body_clone);
    //println!("📨 Raw request body ({} bytes):\n{}", body_clone.len(), body_str);

    // 3. 尝试解析
    match web::Json::<LLOneBot>::from_request(&req, &mut body.into()).await {
        Ok(valid_info) => {
            println!("✅ Parsed successfully: {:#?}", valid_info);
            match handle_message_pipeline(valid_info.into_inner()).await {
                Ok(sendback) => {
                    if let Err(e) = QQ_SENDER.send_qq_post(&sendback).await {
                        eprintln!("🚨 Failed to send QQ post: {:?}", e);
                    }
                    HttpResponse::Ok().body("Success")
                }
                Err(e) => {
                    eprintln!("🚨 Pipeline error: {:?}", e);
                    HttpResponse::InternalServerError().body("Internal server error")
                }
            }
        }
        Err(err) => {
            eprintln!("❌ Actix parse error: {:?}", err);
            HttpResponse::BadRequest()
                .content_type("text/plain")
                .body(format!("Invalid request body. Details:\n\nRaw input:\n{}\n\nError:\n{:?}", 
                    body_str, err))
        }
    }
}