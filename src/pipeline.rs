#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::ll_one_bot::interface::{LLOneBot,SendBackIntermediate,SendBack};
use crate::llm_api::interface::{DeepSeek, Response, ROLE, Message};
use crate::config::{get_config,model_url};
use crate::{DATABASE_MANAGER,API_SENDER,QQ_SENDER};
use serde_json::json;
use actix_web::HttpResponse;
use regex::Regex;
use crate::llm_api::interface::MessageContent;
use std::collections::HashSet;

pub async fn handle_message_pipeline(message: LLOneBot) -> Result<SendBack, HttpResponse> {
  validate_message(&message)?;
  let mut deepseek = preprocess_message(&message).await;
  // 简化思考环节，仅添加系统提示
  // apply_system_prompts(&mut deepseek, &message).await?;
  
  let response = process_message(&deepseek).await?;
  let sendback_message = postprocess_message(&message, &response);
  
  log_message(&message, &sendback_message, &response).await;
  Ok(sendback_message)
}


fn validate_message(message: &LLOneBot) -> Result<(), HttpResponse> {
  //验证消息、用户信息等
  Ok(())
}

/*async fn preprocess_message(message: &LLOneBot) -> DeepSeek {
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
}*/
async fn preprocess_message(message: &LLOneBot) -> DeepSeek {
  let dbmanager = DATABASE_MANAGER.get().unwrap();
  // let mut request = DeepSeek::new("deepseek-chat".to_string(), None, None);
  let mut request = DeepSeek::new("doubao-1.5-vision-pro-32k-250115".to_string(), Some(get_config().presence_penalty), Some(get_config().temperature));
  request.add_self_config(message.get_self_id());
  let context = dbmanager.get_context(message).await.unwrap();
  let history_messages: Vec<HistoryMessage> = context.iter().filter_map(|msg| {
      if let Message { 
          role: ROLE::User , // 只处理用户消息,才能反应用户习惯 
          content: MessageContent::PlainText(text) 
      } = msg {
          Some(HistoryMessage {
              content: text.clone(),
              ..Default::default()
          })
      } else {
          None
      }
  }).collect();
  //let features = analyze_context(&history_messages, &message.get_raw_message());
  // if should_guide_conversation(&features) {
  //   let guide_prompt = generate_guide_prompt(message, &features);
  //   request.add_system_message(guide_prompt); // 👈 在这里调用
  // }
  // apply_context_strategy(&mut request, &features);
  // 打印历史消息
  println!("===== 历史消息记录 =====");
  for (i, msg) in history_messages.iter().enumerate() {
      println!("[消息 {}]: {}", i + 1, msg.content);
  }
  println!("===== 共 {} 条历史消息 =====", history_messages.len());
  
  request.extend_message(context);
  request.add_message(Message::new(ROLE::User, message.extract_message_content()));
  request.handle_special_input();
  // println!("Context features: {:?}", features);

  request
}

#[derive(Default,Debug)]
struct HistoryMessage {
  content: String,
}

async fn process_message(message: &DeepSeek) -> Result<Response,HttpResponse>{
  //调用DeepSeek API处理消息
  println!("message:{:?}",message);
  let result = match message.model.as_str(){
    "doubao-1.5-vision-pro-32k-250115" => API_SENDER.send_api_post(model_url::DOUBAO_VISION,message).await,      
    "deepseek-chat" => API_SENDER.send_api_post(model_url::DEEPSEEK,message).await,
    _ => return Err(HttpResponse::BadRequest().body("Invalid model name")),
  };
  if let Ok(response) = result{
    Ok(response)
  }else{
    eprintln!("AN ERROR OCCUR:{:?}",result);
    Err(HttpResponse::InternalServerError().finish())
  }
}


fn postprocess_message(message:&LLOneBot, response: &Response) -> SendBack{
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
