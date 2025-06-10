use actix_web::{post, web, HttpRequest, HttpResponse, Responder, get};
use crate::config;
use crate::{ll_one_bot::interface::*, pipeline::handle_message_pipeline, QQ_SENDER};
use actix_web::FromRequest;
use std::fs;
use std::path::Path;
use crate::SELECTED_MODEL; 
use crate::services::DEEPSEEK_REQUEST_COUNT;
use crate::services::DEEPSEEK_TOKEN_USAGE;
use crate::services::DOUBAO_REQUEST_COUNT;
use crate::services::DOUBAO_TOKEN_USAGE;
use std::sync::atomic::Ordering; 

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
    println!("📨 Raw request body ({} bytes):\n{}", body_clone.len(), body_str);

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
        Ok(_) => {
            config::reload_config();
            println!("reload_config :{:?}",config::get_config());
            HttpResponse::Ok().body("Config updated successfully")
        },
        Err(e) => {
            eprintln!("❌ Failed to write config file: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update config file")
        }
    }
    
}

#[get("/config_new_list")]
pub async fn get_config_new_list() -> impl Responder {
    let config_new_dir = Path::new("./config_new");
    if let Ok(entries) = fs::read_dir(config_new_dir) {
        let config_files: Vec<String> = entries
           .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_file() && path.extension().map(|s| s == "json").unwrap_or(false) {
                    path.file_name().and_then(|s| s.to_str().map(|s| s.to_string()))
                } else {
                    None
                }
            })
           .collect();
        HttpResponse::Ok()
           .content_type("application/json")
           .body(serde_json::to_string(&config_files).unwrap())
    } else {
        HttpResponse::InternalServerError().body("Failed to read config_new directory")
    }
}

// 获取指定的 config_new 配置文件内容
#[get("/config_new/{filename}")]
pub async fn show_new_config(path: web::Path<String>) -> impl Responder {
    let filename = path.into_inner();
    let config_path = format!("./config_new/{}", filename);
    match fs::read_to_string(&config_path) {
        Ok(config_data) => HttpResponse::Ok()
           .content_type("application/json")
           .body(config_data),
        Err(e) => {
            eprintln!("❌ Failed to read config_new file: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to read config_new file")
        }
    }
}

#[post("/update_model")]
pub async fn update_model(payload: web::Json<serde_json::Value>) -> impl Responder {
    if let Some(model) = payload.get("model").and_then(|m| m.as_str()) {
        // 获取 Mutex 的锁并更新模型
        let mut selected_model = SELECTED_MODEL.lock().unwrap();
        *selected_model = model.to_string();
        HttpResponse::Ok().body("Model updated successfully")
    } else {
        HttpResponse::BadRequest().body("Invalid model name")
    }
}

#[get("/usage_stats")]
pub async fn usage_stats() -> impl Responder {
    let deepseek_request_count = DEEPSEEK_REQUEST_COUNT.load(Ordering::Relaxed);
    let deepseek_token_usage = DEEPSEEK_TOKEN_USAGE.load(Ordering::Relaxed);
    let doubao_request_count = DOUBAO_REQUEST_COUNT.load(Ordering::Relaxed);
    let doubao_token_usage = DOUBAO_TOKEN_USAGE.load(Ordering::Relaxed);

    let stats = serde_json::json!({
        "deepseek_request_count": deepseek_request_count,
        "deepseek_token_usage": deepseek_token_usage,
        "doubao_request_count": doubao_request_count,
        "doubao_token_usage": doubao_token_usage
    });

    HttpResponse::Ok()
       .content_type("application/json")
       .body(stats.to_string())
}