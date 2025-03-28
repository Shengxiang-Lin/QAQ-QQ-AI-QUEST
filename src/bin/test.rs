use QAQ::llm_api::interface::*;
use QAQ::services::ClientManager;
use tokio::task::LocalSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 ClientManager 实例
    let local = LocalSet::new();
    local.run_until(async{
        let client_manager = ClientManager::new("https://api.deepseek.com/chat/completions".to_string());

        let mut payload = DeepSeek::new("deepseek-chat".to_string(), None, None);
        payload.add_message(ROLE::User, "你好".to_string());
    
    
        // 发送 POST 请求
        client_manager.send_post(payload).await?;
    
        Ok(())
    }).await

}