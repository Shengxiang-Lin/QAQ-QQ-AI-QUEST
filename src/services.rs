use actix_web::HttpMessage;
use awc::Client;
use serde_json::json;
use crate::config::config;
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

  pub async fn send_post(&self, payload: impl serde::Serialize) -> Result<(), Box<dyn std::error::Error>>{
    let mut res = self.client.post(&self.url)
      .insert_header(("Content-Type", "application/json")) 
      .insert_header(("Authorization", "Bearer ".to_string() + &config::KEY)) 
      .insert_header(("Accept", "application/json"))
      .send_json(&json!(payload))
      .await?;
    let body = res.body().await?;
    let payload = serde_json::from_slice::<serde_json::Value>(&body)?;
    println!("Response: {:?}", payload);
    Ok(())
  }
}