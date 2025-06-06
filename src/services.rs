use reqwest::Client;
use serde_json::json;
use crate::{config, llm_api::interface::Response, ll_one_bot::interface::*};
use std::sync::atomic::{AtomicU64, Ordering};
// 全局变量记录 API 请求数和消耗的 Token 数
pub static DEEPSEEK_REQUEST_COUNT: AtomicU64 = AtomicU64::new(0);
pub static DEEPSEEK_TOKEN_USAGE: AtomicU64 = AtomicU64::new(0);
pub static DOUBAO_REQUEST_COUNT: AtomicU64 = AtomicU64::new(0);
pub static DOUBAO_TOKEN_USAGE: AtomicU64 = AtomicU64::new(0);
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

  pub async fn send_api_post(&self, url: &str, payload: &impl serde::Serialize) -> Result<Response, Box<dyn std::error::Error + Send + Sync>>{
    let key: &str = match url {
      config::model_url::DEEPSEEK => &config::get_config().deepseek_key.as_str(),
      config::model_url::DOUBAO_VISION => config::get_config().doubao_key.as_str(),
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
    // 记录请求数和Token使用量
    match url {
      config::model_url::DEEPSEEK => {
        DEEPSEEK_REQUEST_COUNT.fetch_add(1, Ordering::Relaxed);
        DEEPSEEK_TOKEN_USAGE.fetch_add(response.usage.total_tokens, Ordering::Relaxed);
      },
      config::model_url::DOUBAO_VISION => {
        DOUBAO_REQUEST_COUNT.fetch_add(1, Ordering::Relaxed);
        DOUBAO_TOKEN_USAGE.fetch_add(response.usage.total_tokens, Ordering::Relaxed);
      },
      _ => {}
    }
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