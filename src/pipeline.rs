#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::ll_one_bot::interface::{LLOneBot,SendBackIntermediate,SendBack};
use crate::llm_api::interface::{DeepSeek, Response, ROLE, Message};
use crate::{DATABASE_MANAGER,API_SENDER,QQ_SENDER};
use serde_json::json;

use actix_web::HttpResponse;


pub async fn handle_message_pipeline(message: LLOneBot) 
-> Result<SendBack,HttpResponse> {
  validate_message(&message)?;
  let deepseek = preprocess_message(&message).await;
  println!("deepseek:{:?}",json!(deepseek));
  let response = process_message(&deepseek).await?;
  let sendback_message = postprecess_message(&message, &response);
  println!("sendback_message:{:?}",sendback_message);
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
  let mut request = DeepSeek::new("deepseek-chat".to_string(), None, None);
  let context = dbmanager.get_context(message).await.unwrap();
  request.extend_message(context);
  request.add_message(Message::new(ROLE::User, message.get_raw_message()));
  request
}



async fn process_message(message: &DeepSeek) -> Result<Response,HttpResponse>{
  //调用DeepSeek API处理消息
  let result = API_SENDER.send_api_post(message).await;
  if let Ok(response) = result{
    Ok(response)
  }else{
    println!("AN ERROR OCCUR:{:?}",result);
    Err(HttpResponse::InternalServerError().finish())
  }
}

fn postprecess_message(message:&LLOneBot, response: &Response) -> SendBack{
  //处理QQ回复消息
  let sendback = SendBackIntermediate::from(response);
  match message {
    LLOneBot::Private(message) => sendback.set_user_id(message.user_id),
    LLOneBot::Group(message) => sendback.set_group_id(message.group_id, message.user_id),

  }

}

async fn log_message(message: &LLOneBot, sendback: &SendBack, response: &Response){
  //sqlite记录消息和回复和token
  let dbmanager = DATABASE_MANAGER.get().unwrap();
  dbmanager.insert_all(message, response, sendback).await.unwrap();
}