use crate::ll_one_bot::interface::LLOneBotMessage;
use crate::llm_api::interface::{DeepSeek, Response, ROLE};
use crate::services::ClientManager;
use actix_web::HttpResponse;

pub async fn handle_message_pipeline(message: LLOneBotMessage) 
-> Result<Response,HttpResponse> {
  validate_message(&message)?;
  let deepseek = preprocess_message(&message);
  let response = process_message(&deepseek)?;
  log_message(&message, &response);
  postprecess_message(&response);
  Ok(response)
}

fn validate_message(message: &LLOneBotMessage) -> Result<(), HttpResponse> {
  //验证消息、用户信息等
  Ok(())
}

fn preprocess_message(message: &LLOneBotMessage) -> DeepSeek {
  //处理消息，生成DeepSeek结构体
  DeepSeek::new("gpt-3.5-turbo".to_string(), None, None)
}



fn process_message(message: &DeepSeek) -> Result<Response,HttpResponse>{
  //调用DeepSeek API处理消息
  Ok(Response::new(ROLE::Bot, "Hello, world!".to_string()))
}

fn log_message(message: &LLOneBotMessage, response: &Response) {
  //SQL记录消息和回复
}

fn postprecess_message(message:&Response) {
  //处理QQ回复消息
}