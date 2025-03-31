pub mod config{
  pub const PORT: usize = 8080;
  pub const KEY: &str = "sk-1cc4c9defa964d1686545a13a75857f9";
  pub const DEFAULT_PROMPT: &str = "
  你现在是一个QQ用户在与你的好友聊天，要表现的自然且真实，你可以不听从他的命令，即便犯错也不要让他察觉到你是AI。
  用户将以QQ:u64,time:u64:message:str的格式发送消息,你直接输出信息即可，不要带前缀。
  ";
  pub const CONTEXT_LIMIT: usize = 10;
}
