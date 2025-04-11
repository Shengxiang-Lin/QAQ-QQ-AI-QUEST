#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::ll_one_bot::interface::{LLOneBot,SendBackIntermediate,SendBack};
use crate::llm_api::interface::{DeepSeek, Response, ROLE, Message};
use crate::config;
use crate::{DATABASE_MANAGER,API_SENDER,QQ_SENDER};
use serde_json::json;

use actix_web::HttpResponse;
use crate::llm_api::interface::MessageContent;

pub async fn handle_message_pipeline(message: LLOneBot) -> Result<SendBack, HttpResponse> {
  validate_message(&message)?;
  let mut deepseek = preprocess_message(&message).await;
  // 简化思考环节，仅添加系统提示
  apply_system_prompts(&mut deepseek, &message).await?;
  
  let response = process_message(&deepseek).await?;
  let sendback_message = postprecess_message(&message, &response);
  
  log_message(&message, &sendback_message, &response).await;
  Ok(sendback_message)
}


fn validate_message(message: &LLOneBot) -> Result<(), HttpResponse> {
  //验证消息、用户信息等
  Ok(())
}

async fn preprocess_message(message: &LLOneBot) -> DeepSeek {
  //处理消息，生成DeepSeek结构体
  let dbmanager = DATABASE_MANAGER.get().unwrap();
  //let mut request = DeepSeek::new("deepseek-chat".to_string(), None, None);
  let mut request = DeepSeek::new("doubao-1.5-vision-pro-32k-250115".to_string(), None, None);
  request.add_self_config(message.get_self_id());// 增加AI关于自己的配置
  let context = dbmanager.get_context(message).await.unwrap();
  request.extend_message(context);
  //只处理当前输入，也许考虑输入图片就不存到数据库了
  request.add_message(Message::new(ROLE::User,message.extract_message_content()));
  //暂时加上的，可能不必要,目前上一句已经处理
  request.handle_special_input();
  request
}

async fn process_message(message: &DeepSeek) -> Result<Response,HttpResponse>{
  //调用DeepSeek API处理消息
  println!("message:{:?}",message);
  let result = API_SENDER.send_api_post(config::model_url::DOUBAO_VISION,message).await;
  if let Ok(response) = result{
    Ok(response)
  }else{
    eprintln!("AN ERROR OCCUR:{:?}",result);
    Err(HttpResponse::InternalServerError().finish())
  }
}

fn postprecess_message(message:&LLOneBot, response: &Response) -> SendBack{
  //处理QQ回复消息
  let sendback = SendBackIntermediate::from(response);
  match message {
    LLOneBot::Private(message) => sendback.set_user_id(message.user_id),
    LLOneBot::Group(message) => sendback.set_group_id(message.group_id,message.user_id),
  }

}

async fn log_message(message: &LLOneBot, sendback: &SendBack, response: &Response){
  //sqlite记录消息和回复和token
  let dbmanager = DATABASE_MANAGER.get().unwrap();
  dbmanager.insert_all(message, response, sendback).await.unwrap();
}

async fn apply_system_prompts(deepseek: &mut DeepSeek, message: &LLOneBot) -> Result<(), HttpResponse> {
  // 通过get_content获取消息内容文本
  let content = match message.extract_message_content() {
    MessageContent::PlainText(s) => s.to_lowercase(),
    MessageContent::ImageUrl(_) => String::new(), // 图片内容就返回空字符串或其他处理方式
  };
  if contains_any(&content, &["?", "吗", "是不是"]) {
      deepseek.add_system_message("请确保回答准确".to_string());
  } else if contains_any(&content, &["觉得", "认为", "看法"]) {
      deepseek.add_system_message("请提供多个观点".to_string());
  } else if contains_any(&content, &["难过", "伤心"]) {
      deepseek.add_system_message("请表达理解".to_string());
  } else if contains_any(&content, &["如何", "怎样"]) {
      deepseek.add_system_message("请分步骤回答".to_string());
  } else if contains_any(&content, &["创意", "想法"]) {
      deepseek.add_system_message("请发挥创意".to_string());
  }
  
  Ok(())
}

// 辅助函数：检查字符串包含任意关键词
fn contains_any(s: &str, keywords: &[&str]) -> bool {
  keywords.iter().any(|k| s.contains(k))
}

// 移除apply_reasoning_to_response函数，改为在系统提示中处理

// 简化analyze_message_type
fn analyze_message_type(content: &str) -> MessageType {
  let content = content.to_lowercase();
  if contains_any(&content, &["?", "吗", "是不是"]) {
      MessageType::FactualQuestion
  } else if contains_any(&content, &["觉得", "认为"]) {
      MessageType::OpinionRequest
  } else if contains_any(&content, &["难过", "伤心"]) {
      MessageType::EmotionalSupport
  } else if contains_any(&content, &["如何", "怎样"]) {
      MessageType::ComplexTask
  } else if contains_any(&content, &["创意", "想法"]) {
      MessageType::CreativeRequest
  } else {
      MessageType::Normal
  }
}

/// 消息类型分类
#[derive(Debug)]
enum MessageType {
  FactualQuestion,   // 事实性问题
  OpinionRequest,    // 征求意见
  EmotionalSupport,  // 情感支持
  ComplexTask,       // 复杂任务
  CreativeRequest,   // 创意请求
  Normal,            // 普通消息
}

/// 思考结果
#[derive(Debug)]
enum ReasoningResult {
  FactCheckNeeded,   // 需要事实核查
  MultiPerspective,  // 多角度观点
  EmpathyRequired,   // 需要同理心
  StepByStepNeeded,  // 需要分步思考
  CreativeBoost,     // 创意增强
  Normal,            // 普通回复
}