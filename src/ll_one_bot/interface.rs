use serde::{Serialize, Deserialize};
use crate::llm_api::interface::Response;


#[derive(Serialize,Deserialize, Debug)]
pub struct SenderInfo{
  user_id: u64,
  nickname: String,
  card: String,
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
impl MessageData{
  pub fn get_text(&self) -> String{
    match self{
      MessageData::Text{text} => text.clone(),
      MessageData::Face{id} => id.clone(),
    }
  }
}
#[derive(Serialize,Deserialize,Debug)]
pub struct LLOneBotPrivate{
  pub self_id: u64,
  pub user_id: u64,
  pub time: u64,
  message_id: u64,
  message_seq: u64,
  message_type: String, // private
  sender: SenderInfo,
  pub raw_message: String,
  font: u8,
  sub_type: String, //friend、group、group_self、other
  message: Vec<QQMessage>,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct LLOneBotGroup{
  pub self_id: u64,
  pub user_id: u64,
  pub group_id: u64,
  pub time: u64, 
  message_id: u64,
  message_type: String, // group
  sender: SenderInfo,
  pub raw_message: String,
  font: u8,
  sub_type: String, //friend、group、group_self、other
  message: Vec<QQMessage>,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LLOneBot{
  Group(LLOneBotGroup),
  Private(LLOneBotPrivate),
  
}

impl LLOneBot{
  pub fn get_self_id(&self) -> u64{
    match self{
      LLOneBot::Private(message) => message.self_id,
      LLOneBot::Group(message) => message.self_id,
    }
  }
  pub fn get_time(&self) -> u64{
    match self{
      LLOneBot::Private(message) => message.time,
      LLOneBot::Group(message) => message.time,
    }
  }
  pub fn get_raw_message(&self) -> String{
    match self{
      LLOneBot::Private(message) => message.raw_message.clone(),
      LLOneBot::Group(message) => message.raw_message.clone(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct SendBackPrivate{
  pub user_id: u64,
  pub message: Vec<QQMessage>
}

#[derive(Serialize,Deserialize,Debug)]
pub struct SendBackGroup{
  pub group_id: u64,
  pub message: Vec<QQMessage>
}

#[derive(Serialize,Deserialize,Debug)]
pub struct SendBackIntermediate{ // 用于中间转换
  message: Vec<QQMessage>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SendBack{
  Private(SendBackPrivate),
  Group(SendBackGroup),
}

impl SendBack{
  pub fn get_content(&self) -> String{
    match self{
      SendBack::Private(sendback) => {
        let mut content = String::new();
        for message in &sendback.message {
          content.push_str(&message.data.get_text());
        }
        return content;
      },
      SendBack::Group(sendback) => {
        let mut content = String::new();
        for message in &sendback.message {
          content.push_str(&message.data.get_text());
        }
        return content;
      },
    }
  }
}

impl From<&Response> for SendBackIntermediate{
  fn from(response: &Response) -> Self{
    let mut message = Vec::new();
    // 这里需要加入表情支持
    message.push(QQMessage { 
      r#type: "text".to_string(), 
      data:{
        MessageData::Text{
          text: response.get_content()
        }
      },
    });
    Self{
      message,
    }
  }
}

impl SendBackIntermediate{ // 中间件，用完即消失
  pub fn set_user_id(self, user_id: u64) -> SendBack {
    SendBack::Private(SendBackPrivate {
      user_id,
      message: self.message,
    })
  }
  pub fn set_group_id(self, group_id: u64)-> SendBack{
    SendBack::Group(SendBackGroup{
      group_id,
      message: self.message
    })
  }
} 
