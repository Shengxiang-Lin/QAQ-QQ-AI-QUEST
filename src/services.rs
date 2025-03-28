use actix_web::HttpMessage;
use reqwest::Client;
use once_cell::sync::Lazy;
use serde_json::json;
use crate::{config::config, llm_api::interface::Response, ll_one_bot::interface::SendBackMessage};

//同一二进制文件下使用crate，不同二进制文件下使用QAQ，因为都在lib.rs中声明了模块，故用crate
pub struct ClientManager{
  client: Client,
  url: String,
}

impl ClientManager{
  pub fn new(url: String) -> Self {
    Self {
        client: Client::new(),
        url,
    }
}

  pub async fn send_api_post(&self, payload: &impl serde::Serialize) -> Result<Response, Box<dyn std::error::Error>>{
    let res = self.client.post(&self.url)
      .header("Content-Type", "application/json") 
      .header("Authorization", "Bearer ".to_string() + &config::KEY) 
      .header("Accept", "application/json")
      .json(&json!(payload))
      .send()
      .await?;
    let response = res.json::<Response>().await?;
    
    println!("Response: {:?}", response);
    Ok(response)
  }

  pub async fn send_qq_post(&self, payload: &SendBackMessage) -> Result<(),Box<dyn std::error::Error>>{
    let res = self.client.post(&self.url)
    .json(&json!(payload))
    .send()
    .await?;
    let response = res.text().await?;
    println!("Response: {:?}", response);
    Ok(())
  }
}


pub static API_SENDER: Lazy<ClientManager> = Lazy::new(|| {
  ClientManager::new("https://api.deepseek.com/chat/completions".to_string())
});

pub static QQ_SENDER: Lazy<ClientManager> = Lazy::new(|| {
  ClientManager::new("http://localhost:3000/send_private_msg".to_string())
});
