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
use crate::SELECTED_MODEL;

pub async fn handle_message_pipeline(message: LLOneBot) -> Result<SendBack, HttpResponse> {
  // validate_message(&message)?;
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
  let qqids:&Vec<u64> = &get_config().valid_qq;
  let user_id = message.get_user_id();
  if !qqids.contains(&user_id) {
    return Err(HttpResponse::BadRequest().body("Invalid user ID"));
  }
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
//智能话题引导
fn should_guide_conversation(features: &ContextFeatures) -> bool {
  features.topic_consistency < get_config().topic_guide_threshold
}

async fn preprocess_message(message: &LLOneBot) -> DeepSeek {
  let dbmanager = DATABASE_MANAGER.get().unwrap();
  let model_name = {
    let selected_model = SELECTED_MODEL.lock().unwrap();
    selected_model.clone()
  };
  let mut request = DeepSeek::new(model_name, Some(get_config().presence_penalty), Some(get_config().temperature));
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
  request.extend_message(context);
  request.add_message(Message::new(ROLE::User, message.extract_message_content()));
  request.handle_special_input();
  request
}

#[derive(Default,Debug)]
struct HistoryMessage {
  content: String,
  // 其他字段不需要实际使用
}

#[derive(Default,Debug)]
struct ContextFeatures {
  avg_length: usize,
  is_deep_discussion: bool,
  emotion_tone: i32,
  topic_consistency: f32,
  avg_emoji_count: f32,
}

fn analyze_context(messages: &[HistoryMessage], current_message: &String) -> ContextFeatures {
  let mut features = ContextFeatures::default();
  if messages.is_empty() {
      return features;
  }
  features.avg_emoji_count = messages.iter()
    .map(|m| m.content.matches("CQ:face").count() as f32)
    .sum::<f32>() / messages.len() as f32;
  // 分析消息长度特征
  features.avg_length = messages.iter()
      .map(|m| m.content.len())
      .sum::<usize>() / messages.len();
  // 检测讨论深度
  features.is_deep_discussion = messages.iter()
      .any(|m| m.content.len() > 100 || 
           m.content.contains("为什么") || 
           m.content.contains("分析"));
  // 检测情感倾向
  let positive_words = ["好", "开心", "谢谢", "喜欢"];
  let negative_words = ["生气", "讨厌", "难受", "不好"];
  features.emotion_tone = messages.iter()
      .fold(0, |acc, m| {
          acc + positive_words.iter().filter(|&w| m.content.contains(w)).count() as i32
          - negative_words.iter().filter(|&w| m.content.contains(w)).count() as i32
      });
  // 检测话题集中度
  if messages.len() >= 3 {
      let last_3_msg_keywords = messages.iter().rev().take(3)
          .flat_map(|m| extract_keywords(&m.content))
          .collect::<Vec<_>>();
      println!("Last 3 message keywords: {:?}", last_3_msg_keywords);
      features.topic_consistency = last_3_msg_keywords.iter()
          .filter(|&kw| current_message.contains(kw))
          .count() as f32 / last_3_msg_keywords.len() as f32;
  }
  features
}

fn apply_context_strategy(deepseek: &mut DeepSeek, features: &ContextFeatures) {
  // 深度讨论模式
  if features.avg_emoji_count < 0.5 {
    deepseek.add_system_message("用户不偏好使用表情符号，回答时适当减少使用表情".to_string());
  }
  if features.is_deep_discussion {
      deepseek.add_system_message(
          "检测到深度讨论上下文，请：\n\
           - 保持逻辑连贯性\n\
           - 引用之前讨论的关键点\n\
           - 允许适度的理论深度".to_string()
      );
  }
  // 情感响应模式
  match features.emotion_tone {
      x if x > 2 => deepseek.add_system_message("检测到积极情绪，请匹配愉快语气并适当使用表情符号".to_string()),
      x if x < -2 => deepseek.add_system_message("检测到负面情绪，请先表达共情再提供建议".to_string()),
      _ => {}
  }
  // 长文本模式
  if features.avg_length > 100 {
      deepseek.add_system_message("用户偏好详细回复，回复长一些".to_string());
  }else{
    deepseek.add_system_message("用户偏好较短聊天，回复更像短聊天".to_string());
  }
  // 话题一致性提示
  if features.topic_consistency > get_config().topic_continue_threshold {
      deepseek.add_system_message("当前话题高度集中，请保持回答的相关性".to_string());
  }
}

fn extract_keywords(content: &str) -> Vec<String> {
  // 只要 message后面部分
  let re = Regex::new(r"message:(.*)").unwrap();
  if let Some(captures) = re.captures(content) {
      if let Some(message) = captures.get(1) {
          // 提取 message 部分的内容
          let message_content = message.as_str();
          
          // 分词并过滤停用词
          let stop_words = vec![r"[\s.,!?。，；：“”‘’]","的", "了", "是", "我", "你", "啊", "还", "不", "巨","个","像","就","这","那","有","要","吗","吧","CQ","id"];
          let stop_words_pattern = stop_words.join("|");
          let keywords = re
            .split(message_content) 
            .filter(|word| word.len() > 2); // 过滤长度小于等于2的词和停用词

          // 生成2-3字的关键词组合
          let mut refined_keywords = Vec::new();
          for keyword in keywords {
              let chars: Vec<char> = keyword.chars().collect();
              for i in 0..chars.len() {
                  if i + 2 <= chars.len() {
                      refined_keywords.push(chars[i..i + 2].iter().collect());
                  }
                  if i + 3 <= chars.len() {
                      refined_keywords.push(chars[i..i + 3].iter().collect());
                  }
              }
          }
            refined_keywords

      } else {
          vec![]
      }
  } else {
      vec![]
  }
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

fn generate_guide_prompt(message: &LLOneBot, features: &ContextFeatures) -> String {
  match message {
      LLOneBot::Private(_) => "检测到话题分散，建议主动引导：\n- 提供2-3个相关讨论方向\n- 使用『你是不是想说...』等开放式提问".to_string(),
      LLOneBot::Group(_) => "检测到群聊话题分散，建议：\n- 总结当前讨论要点\n- 提出投票式问题『大家更关注A还是B？』".to_string()
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

async fn apply_system_prompts(deepseek: &mut DeepSeek, message: &LLOneBot) -> Result<(), HttpResponse> {
  let content = match message.extract_message_content() {
      MessageContent::PlainText(s) => s.to_lowercase(),
      MessageContent::ImageUrl(_) => String::new(),
  };
  // 首先分析消息类型
  let msg_type = analyze_message_type(&content);
  println!("Message type: {:?}", msg_type);
  // 根据消息类型添加不同的系统提示和思考要求
  match msg_type {
      MessageType::FactualQuestion => {
          deepseek.add_system_message(
              "请按照以下步骤思考并回答：
              1. 仔细分析问题中的关键事实要素
              2. 验证你掌握的相关知识是否准确可靠
              3. 考虑问题可能存在的多种解释或答案
              4. 提供最可能的答案并说明依据
              5. 如果存在不确定性，明确说明并给出可能的方向"
              .to_string()
          );
      }
      MessageType::OpinionRequest => {
          deepseek.add_system_message(
              "请按照以下框架提供观点：
              1. 首先分析问题的各个相关方立场
              2. 列举支持每个立场的主要论据
              3. 评估不同观点的优缺点
              4. 提供你自己的综合判断
              5. 说明你的判断标准是什么"
              .to_string()
          );
      }
      MessageType::EmotionalSupport => {
          deepseek.add_system_message(
              "请按此流程回应情感需求：
              1. 首先识别并确认对方的情绪状态
              2. 表达真诚的理解和共情
              3. 询问是否需要具体建议
              4. 如果对方愿意接受，提供温和的支持性建议
              5. 保持非评判态度，给予情感支持"
              .to_string()
          );
      }
      MessageType::ComplexTask => {
          deepseek.add_system_message(
              "请按结构化方式指导：
              1. 将复杂任务分解为关键步骤
              2. 为每个步骤提供详细说明和技巧
              3. 指出可能遇到的困难及解决方案
              4. 提供可选的替代方案
              5. 总结完成后的预期结果"
              .to_string()
          );
      }
      MessageType::CreativeRequest => {
          deepseek.add_system_message(
              "请按创新思维流程：
              1. 首先突破常规思维，列出疯狂想法
              2. 筛选出最具潜力的3个方向
              3. 为每个方向构思具体实施方案
              4. 评估每个方案的可行性和创新性
              5. 推荐最佳方案并说明理由"
              .to_string()
          );
      }
      MessageType::Normal => {
          deepseek.add_system_message(
              "请按深度交流原则回应：
              1. 分析消息背后的潜在需求
              2. 考虑相关背景和上下文
              3. 提供有见地的观点或信息
              4. 以促进对话深入为目标
              5. 保持友好专业的语气"
              .to_string()
          );
      }
  }
  // 添加通用深度思考提示
  deepseek.add_system_message(
      "在回答前，请先进行以下思考：
      1. 这个问题涉及哪些核心概念？
      2. 有哪些相关因素需要考虑？
      3. 是否存在不同的视角或解释？
      4. 我的回答可能产生什么影响？
      5. 如何使这个回答更有价值和深度？"
      .to_string()
  );
  Ok(())
}
// 辅助函数：检查字符串包含任意关键词
fn contains_any(s: &str, keywords: &[&str]) -> bool {
  keywords.iter().any(|k| s.contains(k))
}

// 消息类型分析
fn analyze_message_type(content: &str) -> MessageType {
  let content = content.to_lowercase();
  // 事实性问题检测
  if contains_any(&content, &["?", "吗", "是不是", "是否正确", "是否应该", "真伪"]) 
      && (contains_any(&content, &["事实", "数据", "统计", "研究", "证明"]) 
          || !contains_any(&content, &["觉得", "认为"])) {
      return MessageType::FactualQuestion;
  }
  // 情感支持检测
  if contains_any(&content, &["难过", "伤心", "抑郁", "孤独", "焦虑", "压力", "崩溃"]) 
      || (contains_any(&content, &["怎么办", "帮助"]) 
          && contains_any(&content, &["我", "自己"])) {
      return MessageType::EmotionalSupport;
  }
  // 复杂任务检测
  if contains_any(&content, &["如何", "怎样", "步骤", "方法", "流程"]) 
      && (content.len() > 15 || contains_any(&content, &["复杂", "困难", "不会"])) {
      return MessageType::ComplexTask;
  }
  // 创意请求检测
  if contains_any(&content, &["创意", "想法", "灵感", "创新", "新颖", "独特"]) 
      || contains_any(&content, &["建议", "推荐"]) && contains_any(&content, &["有趣的", "特别的"]) {
      return MessageType::CreativeRequest;
  }
  // 观点请求检测
  if contains_any(&content, &["觉得", "认为", "看法", "观点", "你怎么看", "你怎么想"]) {
      return MessageType::OpinionRequest;
  }
  MessageType::Normal
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