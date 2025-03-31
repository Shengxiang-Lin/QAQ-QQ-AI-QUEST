pub mod config;
pub mod services;
pub mod routes;
pub mod handlers;
pub mod db;
pub mod pipeline;

use chrono::TimeZone;
use chrono::{DateTime, Utc,FixedOffset};
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;

pub mod ll_one_bot{
  pub mod interface;
}

pub mod llm_api{
  pub mod interface{
    use serde::{Serialize, Deserialize};
    use crate::config::config;

    #[derive(Serialize,Deserialize,Debug,PartialEq)]
    #[serde(rename_all = "lowercase")] // 将枚举值序列化为小写字符串
    pub enum ROLE{
      System,
      User,
      Assistant,
    }

    #[derive(Serialize,Deserialize,Debug)]
    pub struct Message{
      pub role: ROLE,
      pub content: String,
    }

    impl Message{
      pub fn new(role: ROLE, content: String)->Self{
        Self{
          role,
          content,
        }
      }
    }


    #[derive(Serialize,Deserialize,Debug)]
    pub struct DeepSeek{
      model: String,
      messages: Vec<Message>,
      presence_penalty: f32, // 介于-2 ~ 2之间，越大越容易转移话题
      temperature: f32, // 介于0 ~ 2之间，越大越随机
    }

    impl DeepSeek{
      pub fn new(model: String, presence_penalty: Option<f32>, temperature: Option<f32>)->Self{
        let mut message = Vec::new();
        message.push(Message::new(ROLE::System, config::DEFAULT_PROMPT.to_string()));
        Self{
          model,
          messages: message,
          presence_penalty: presence_penalty.unwrap_or(0.0),
          temperature: temperature.unwrap_or(1.0),
        }
      }

      pub fn add_system_message(&mut self, content: String){
        let mut count:usize = 0;
        for i in self.messages.iter(){
          if i.role != ROLE::System{
            count += 1;
          }else{
            break;
          }
        }
        self.messages.insert(count, Message::new(ROLE::System, content));
      }

      pub fn extend_message(&mut self, vec: Vec<Message>){
        self.messages.extend(vec);
      }

      pub fn add_message(&mut self, message: Message){
        self.messages.push(message);
      }
      
    }

    #[derive(Serialize,Deserialize,Debug)]
    pub struct Response{
      choices: Vec<Choice>,
      created: u64,
      id: String,
      model: String,
      object: String,
      system_fingerprint: String,
      pub usage: Usage,
    }

    #[derive(Serialize,Deserialize,Debug)]
    pub struct Choice{
      finish_reason: String,
      index: u64,
      logprobs: Option<serde_json::Value>,
      message: Message,
    }

    #[derive(Serialize,Deserialize,Debug)]
    pub struct Usage{
      pub completion_tokens: u64,
      pub prompt_cache_hit_tokens: u64,
      prompt_cache_miss_tokens: u64,
      pub prompt_tokens: u64,
      prompt_tokens_details: PromptTokensDetails,
      pub total_tokens: u64,
    }

    #[derive(Serialize,Deserialize,Debug)]
    pub struct PromptTokensDetails{
      cached_tokens: u64,
    }

    impl Response{
      pub fn get_content(&self)->String{
        self.choices[0].message.content.clone()
      }
    }
  }
}

pub async fn initialize_database_manager() {
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let db_manager = db::DatabaseManager::new(&database_url).await.expect("Failed to initialize DatabaseManager");
  DATABASE_MANAGER.set(db_manager);
}

pub static DATABASE_MANAGER: OnceCell<db::DatabaseManager> = OnceCell::new();

pub static API_SENDER: Lazy<services::ClientManager> = Lazy::new(|| {
  services::ClientManager::new()
});

pub static QQ_SENDER: Lazy<services::ClientManager> = Lazy::new(|| {
  services::ClientManager::new()
});

pub fn second2date(seconds: i64) -> String {
    // 使用 Utc.timestamp_opt 替代 Utc.timestamp
    let datetime_utc = Utc.timestamp_opt(seconds, 0).single().expect("Invalid timestamp");

    // 转换为东八区时间
    let offset = FixedOffset::east_opt(8 * 3600).expect("Invalid offset");
    let datetime_east8 = datetime_utc.with_timezone(&offset);

    // 格式化为字符串
    datetime_east8.format("%Y-%m-%d %H:%M:%S").to_string()
}