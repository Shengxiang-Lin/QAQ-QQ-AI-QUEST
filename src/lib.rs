pub mod config;
pub mod services;
pub mod routes;
pub mod handlers;

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

      pub fn add_message(&mut self, role: ROLE, content: String){
        self.messages.push(Message::new(role, content));
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

