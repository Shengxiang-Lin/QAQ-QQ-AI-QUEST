use actix_web::HttpMessage;
use awc::Client;
use once_cell::sync::Lazy;
use serde_json::json;
use crate::{config::config, llm_api::interface::Response};

//同一二进制文件下使用crate，不同二进制文件下使用QAQ，因为都在lib.rs中声明了模块，故用crate
pub struct ClientManager{
  client: Client,
  url: String,
}

impl ClientManager{
  pub fn new(url: String)->Self{
    Self{
      client: Client::default(),
      url,
    }
  }

  pub async fn send_post(&self, payload: impl serde::Serialize) -> Result<Response, Box<dyn std::error::Error>>{
    let mut res = self.client.post(&self.url)
      .insert_header(("Content-Type", "application/json")) 
      .insert_header(("Authorization", "Bearer ".to_string() + &config::KEY)) 
      .insert_header(("Accept", "application/json"))
      .send_json(&json!(payload))
      .await?;
    let body = res.body().await?;
    let response: Response = serde_json::from_slice(&body)?;
    
    println!("Response: {:?}", response);
    Ok(response)
  }
}


pub static CLIENT_MANAGER: Lazy<ClientManager> = Lazy::new(|| {
  ClientManager::new("https://api.deepseek.com/chat/completions".to_string())
});
