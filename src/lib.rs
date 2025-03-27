pub mod LLOneBot{
  pub mod interface{
    #[derive(Deserialize, Debug)]
    pub struct SenderInfo{
      pub user_id: u64,
      pub nickname: String,
      pub card: String,
    }

    #[derive(Deserialize,Debug)]
    pub struct QQMessage{

    }

    #[derive(Deserialize,Debug)]
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

    }
  }
}