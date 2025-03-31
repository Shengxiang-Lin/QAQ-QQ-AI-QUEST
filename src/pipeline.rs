use crate::ll_one_bot::interface::{LLOneBot,SendBackIntermediate,SendBack};
use crate::llm_api::interface::{DeepSeek, Response, ROLE};
use crate::services::ClientManager;
use actix_web::HttpResponse;

pub async fn handle_message_pipeline(message: LLOneBot) 
-> Result<SendBack,HttpResponse> {
  validate_message(&message)?;
  let deepseek = preprocess_message(&message);
  let response = process_message(&deepseek)?;
  let sendback_message = postprecess_message(&message, &response);
  log_message(&message, &send_back_message, &response);
  Ok(send_back_message)
}

fn validate_message(message: &LLOneBot) -> Result<(), HttpResponse> {
  //验证消息、用户信息等
  Ok(())
}

fn preprocess_message(message: &LLOneBot) -> DeepSeek {
  //处理消息，生成DeepSeek结构体
  DeepSeek::new("gpt-3.5-turbo".to_string(), None, None)
}



fn process_message(message: &DeepSeek) -> Result<Response,HttpResponse>{
  //调用DeepSeek API处理消息
  Ok(Response::new(ROLE::Bot, "Hello, world!".to_string()))
}

fn postprecess_message(message:&LLOneBot, response: &Response) -> SendBack{
  //处理QQ回复消息
}

fn log_message(message: &LLOneBot, sendback: &SendBack, response: &Response){
  //sqlite记录消息和回复和token
  
}