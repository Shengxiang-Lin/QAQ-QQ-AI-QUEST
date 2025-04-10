use reqwest::Client;
use serde_json::json;
use crate::{config, llm_api::interface::Response, ll_one_bot::interface::*};

//同一二进制文件下使用crate，不同二进制文件下使用QAQ，因为都在lib.rs中声明了模块，故用crate
pub struct ClientManager{
  client: Client,
}

impl ClientManager{
  pub fn new() -> Self {
    Self {
        client: Client::new(),
    }
}

  pub async fn send_api_post(&self, url: &str, payload: &impl serde::Serialize) -> Result<Response, Box<dyn std::error::Error>>{
    let key: &str = match url {
      config::model_url::DEEPSEEK => config::config::KEY,
      config::model_url::DOUBAO_VISION => config::config::DOUBAO_KEY,
      _ => panic!("Invalid URL"),
    };
    let res = self.client.post(url)
      .header("Content-Type", "application/json") 
      .header("Authorization", "Bearer ".to_string() + key) 
      .header("Accept", "application/json")
      .json(&json!(payload))
      .send()
      .await?;
    let response = res.json::<Response>().await?;
    println!("Response: {:?}", response);
    Ok(response)
  }

  pub async fn send_qq_post(&self, payload: &SendBack) -> Result<(),Box<dyn std::error::Error>>{
    let url = match payload{
      SendBack::Private(_) => "http://localhost:3000/send_private_msg".to_string(),
      SendBack::Group(_) =>"http://localhost:3000/send_group_msg".to_string(),
    };
    let res = self.client.post(url)
    .json(&json!(payload))
    .send()
    .await?;
    Ok(())
  }
}



