use actix_web::{post, web, HttpRequest, HttpResponse, Responder, get};
use crate::{ll_one_bot::interface::*, pipeline::handle_message_pipeline, QQ_SENDER};
use actix_web::FromRequest;
use std::fs;

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
    // println!("📨 Raw request body ({} bytes):\n{}", body_clone.len(), body_str);

    // 3. 尝试解析
    match web::Json::<LLOneBot>::from_request(&req, &mut body.into()).await {
        Ok(valid_info) => {
            // println!("✅ Parsed successfully: {:#?}", valid_info);
            match handle_message_pipeline(valid_info.into_inner()).await {
                Ok(sendback) => {
                    if let Err(e) = QQ_SENDER.send_qq_post(&sendback).await {
                        eprintln!("🚨 Failed to send QQ post: {:?}", e);
                    }
                    HttpResponse::Ok().body("Success")
                }
                Err(e) => {
                    eprintln!("🚨 Pipeline error: {:?}", e.body());
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

#[get("/config")]
pub async fn show_config() -> impl Responder {
    match fs::read_to_string("./config.json") {
        Ok(config_data) => HttpResponse::Ok()
            .content_type("application/json")
            .body(config_data),
        Err(e) => {
            eprintln!("❌ Failed to read config file: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to read config file")
        }
    }
}

#[post("/update_config")]
pub async fn update_config(payload: web::Json<serde_json::Value>) -> impl Responder {
    let new_config = payload.into_inner();
    match fs::write("./config.json", serde_json::to_string_pretty(&new_config).unwrap()) {
        Ok(_) => HttpResponse::Ok().body("Config updated successfully"),
        Err(e) => {
            eprintln!("❌ Failed to write config file: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update config file")
        }
    }
}