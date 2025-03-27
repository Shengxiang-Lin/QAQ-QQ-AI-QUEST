pub mod ll_one_bot{
  pub mod interface{
    #[derive(serde::Serialize,serde::Deserialize, Debug)]
    pub struct SenderInfo{
      pub user_id: u64,
      pub nickname: String,
      pub card: String,
    }

    #[derive(serde::Serialize,serde::Deserialize,Debug)]
    pub struct QQMessage{
      pub r#type : String,
      pub data: MessageData
    }
    
    #[derive(serde::Serialize,serde::Deserialize,Debug)]
    #[serde(untagged)] // 使其反序列化时匹配内部类型而非枚举类型Text/Face
    pub enum MessageData{
      Text{text: String},
      Face{id: String},
    }

    #[derive(serde::Serialize,serde::Deserialize,Debug)]
    pub struct LLOneBotMessage{
      pub self_id: u64,
      pub user_id: u64,
      pub time: u64,
      pub message_id: u64,
      pub message_seq: u64,
      pub message_type: String,
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