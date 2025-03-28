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
  r#type : String,
  data: MessageData
}

#[derive(Serialize,Deserialize,Debug)]
#[serde(untagged)] // 使其反序列化时匹配内部类型而非枚举类型Text/Face
pub enum MessageData{
  Text{text: String},
  Face{id: String},
}

#[derive(Serialize,Deserialize,Debug)]
pub struct LLOneBotMessage{
  self_id: u64,
  user_id: u64,
  time: u64,
  message_id: u64,
  message_seq: u64,
  message_type: String, // private/group
  sender: SenderInfo,
  raw_message: String,
  font: u8,
  sub_type: String,
  message: Vec<QQMessage>,
  message_format: String,
  post_type: String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct SendBackMessage{
  user_id: u64,
  message: Vec<QQMessage>
}

impl From<&Response> for SendBackMessage{
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
      user_id: 0,
      message,
    }
  }
}

impl SendBackMessage{
  pub fn set_user_id(&mut self, user_id: u64){
    self.user_id = user_id;
  }
} 