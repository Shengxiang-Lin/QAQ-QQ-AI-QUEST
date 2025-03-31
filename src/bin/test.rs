use QAQ::{ll_one_bot::interface::SendBackIntermediate, llm_api::interface::*};
use QAQ::services::{API_SENDER, QQ_SENDER};
use tokio::task::LocalSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 ClientManager 实例
    let local = LocalSet::new(); // 奇怪报错spawn_local called from outside of a task::LocalSet or LocalRuntime的解决方法
    local.run_until(async{

        let mut payload = DeepSeek::new("deepseek-chat".to_string(), None, None);
        payload.add_message(ROLE::User, "你好".to_string());
        
    
        // 发送 POST 请求
        let response = API_SENDER.send_api_post(&payload).await?;
        let message = SendBackIntermediate::from(&response);
        let message = message.set_user_id(2421468125);
        QQ_SENDER.send_qq_post(&message).await?;
        Ok(())
    }).await

}