pub mod config;
pub mod services;
pub mod ll_one_bot{
  pub mod interface{
    use serde::{Serialize, Deserialize};

    #[derive(Serialize,Deserialize, Debug)]
    pub struct SenderInfo{
      pub user_id: u64,
      pub nickname: String,
      pub card: String,
    }

    #[derive(Serialize,Deserialize,Debug)]
    pub struct QQMessage{
      pub r#type : String,
      pub data: MessageData
    }
    
    #[derive(Serialize,Deserialize,Debug)]
    #[serde(untagged)] // 使其反序列化时匹配内部类型而非枚举类型Text/Face
    pub enum MessageData{
      Text{text: String},
      Face{id: String},
    }

    #[derive(Serialize,Deserialize,Debug)]
    pub struct LLOneBotMessage{
      pub self_id: u64,
      pub user_id: u64,
      pub time: u64,
      pub message_id: u64,
      pub message_seq: u64,
      pub message_type: String, // private/group
      pub sender: SenderInfo,
      pub raw_message: String,
      pub font: u8,
      pub sub_type: String,
      pub message: Vec<QQMessage>,
      pub message_format: String,
      pub post_type: String,
    }
  }
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
      pub model: String,
      pub messages: Vec<Message>,
      pub presence_penalty: f32, // 介于-2 ~ 2之间，越大越容易转移话题
      pub temperature: f32, // 介于0 ~ 2之间，越大越随机
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

  }
}

