use sqlx::{Pool,Sqlite,SqlitePool};
use std::sync::Arc;
use QAQ::config::config;
use crate::ll_one_bot::interface::{LLOneBot,SendBack};
use crate::llm_api::interface::Response;

pub struct Database{
  pub pool: Arc<SqlitePool>,
}


impl Database{
  pub async fn new(database_url: &str) -> Result<Self, sqlx::Error>{
    let pool = SqlitePool::connect(database_url).await?;
    let db = Database{
      pool: Arc::new(pool),
    };
    db.create_tables().await?;
    Ok(db)
  }

  async fn create_tables(&self) -> Result<(), sqlx::Error>{
    sqlx::query(MESSAGE_TABLE).execute(&*self.pool).await?;
    sqlx::query(RESPONSE_TABLE).execute(&*self.pool).await?;
    sqlx::query(USAGE_TABLE).execute(&*self.pool).await?;
    Ok(())
  }

  pub async fn insert_message(
    &self, 
    self_id: u64, 
    user_id: u64, 
    group_id: Option<u64>,
    time: u64, 
    raw_message: &str
  ) -> Result<(), sqlx::Error>{
    sqlx::query!(r#"
      INSERT INTO message (self_id, user_id, group_id, time, raw_message)
      VALUES (?, ?, ?, ?, ?)
      "#,self_id,user_id,group_id,time,raw_message)
      .execute(&*self.pool)
      .await?;
    Ok(())
  } 
  
  pub async fn insert_response(
    &self, 
    self_id: u64,
    user_id: Option<u64>, 
    group_id: Option<u64>,
    raw_message: &str, 
    time: u64,
  ) -> Result<(), sqlx::Error>{
    sqlx::query!(r#"
      INSERT INTO response (self_id, user_id, group_id, raw_message, time, token)
      VALUES (?, ?, ?, ?, ?)
      "#,self_id,user_id,group_id,raw_message,time)
      .execute(&*self.pool)
      .await?;
    Ok(())
  }


  pub async fn insert_usage(
    &self, 
    response_id: u64, 
    total_tokens: u64, 
    prompt_tokens: u64, 
    prompt_cache_hit_tokens: u64, 
    completion_tokens: u64
  ) -> Result<(), sqlx::Error>{
    sqlx::query!(r#"
      INSERT INTO usage_stats (response_id, total_tokens, prompt_tokens, prompt_cache_hit_tokens, completion_tokens)
      VALUES (?, ?, ?, ?, ?)
      "#,response_id,total_tokens,prompt_tokens,prompt_cache_hit_tokens,completion_tokens)
      .execute(&*self.pool)
      .await?;
    Ok(())
  }

  // 返回<qq号，消息，时间>
  pub async fn get_private_context(&self, user_id: u64) -> Result<Vec<(u64, String, u64)>, sqlx::Error> {
    let messages = sqlx::query!(
        r#"
        SELECT time, raw_message, user_id AS id
        FROM message
        WHERE user_id = ?
        LIMIT ?
        "#,
        user_id, config::CONTEXT_LIMIT)
    .fetch_all(&*self.pool)
    .await?;
    let responses = sqlx::query!(
      r#"
      SELECT time, raw_message self_id AS id
      FROM response
      WHERE user_id = ?
      LIMIT ?
      "#,
      user_id,config::CONTEXT_LIMIT
    )
    .fetch_all(&*self.pool)
    .await?;
    let mut combined: Vec<(u64, String, u64)> = Vec::new();  
    for message in messages {
      combined.push((message.id, message.raw_message, message.time));
    }

    for response in responses {
      combined.push((response.id, response.raw_message, response.time));
    }

    combined.sort_by(|a, b| b.2.cmp(&a.2));
    let limited = combined.into_iter().take(config::CONTEXT_LIMIT).collect();
    Ok(limited)
  }

  pub async fn get_group_context(&self, group_id: u64) -> Result<Vec<(u64, String, u64)>, sqlx::Error> {
    let messages = sqlx::query!(
        r#"
        SELECT time, raw_message, user_id AS id
        FROM message
        WHERE group_id = ?
        LIMIT ?
        "#,
        group_id, config::CONTEXT_LIMIT)
    .fetch_all(&*self.pool)
    .await?;
    let responses = sqlx::query!(
      r#"
      SELECT time, raw_message self_id AS id
      FROM response
      WHERE group_id = ?
      LIMIT ?
      "#,
      group_id,config::CONTEXT_LIMIT
    )
    .fetch_all(&*self.pool)
    .await?;
    let mut combined: Vec<(u64, String, u64)> = Vec::new();  
    for message in messages {
      combined.push((message.id, message.raw_message, message.time));
    }

    for response in responses {
      combined.push((response.id, response.raw_message, response.time));
    }

    combined.sort_by(|a, b| b.2.cmp(&a.2));
    let limited = combined.into_iter().take(config::CONTEXT_LIMIT).collect();
    Ok(limited)
  }


}

pub struct DatabaseManager{
  pub db: Database,
  //预留缓存

}

impl DatabaseManager{
  pub async fn new(database_url: &str) -> Result<Self, sqlx::Error>{
    let db = Database::new(database_url).await?;
    Ok(DatabaseManager{
      db,
    })
  }

  pub async fn insert_all(&self, message: &LLOneBot, response: &Response) -> Result<(), sqlx::Error>{
    let id = self.insert_message_and_sendback(message, &response.into()).await?;
    self.insert_token_usage(1, response).await?;
    Ok(())
  }

  async fn insert_message_and_sendback(&self, message: &LLOneBot, response: &SendBack) ->Result<u64, sqlx::Error>{
    match message{
      LLOneBot::Private(message) =>{
        self.db.insert_message(
          message.self_id,
          message.user_id,
          None,
          message.time,
          message.raw_message.as_str()
        )
      }
      LLOneBot::Group(message) =>{
        self.db.insert_message(
          message.self_id,
          message.user_id,
          Some(message.group_id),
          message.time,
          message.raw_message.as_str()
        )
      }
    }
    
    let raw_message: &str = response.get_content().as_str();
    let response_id = match response{
      SendBack::Private(response) =>{
        self.db.insert_response(
          message.self_id,
          Some(response.user_id),
          None,
          raw_message,
          message.time,
        ).await?
      }
      SendBack::Group(response) =>{
        self.db.insert_response(
          message.self_id,
          None,
          Some(response.group_id),
          raw_message,
          message.time,
        ).await?
      }
    };
    Ok(response_id)
  }

  async fn insert_token_usage(&self, id: u64, response: &Response) -> Result<(), sqlx::Error>{
    self.db.insert_usage(
      id,
      response.usage.total_tokens,
      response.usage.prompt_tokens,
      response.usage.prompt_cache_hit_tokens,
      response.usage.completion_tokens
      )
  }
}




static MESSAGE_TABLE : &str = r#"
CREATE TABLE IF NOT EXISTS message (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    self_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    group_id INTEGER,
    time INTEGER NOT NULL,
    raw_message TEXT NOT NULL,
);"#;

static RESPONSE_TABLE : &str = r#"
CREATE TABLE IF NOT EXISTS response (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    self_id INTEGER NOT NULL,
    user_id INTEGER,
    group_id INTEGER,
    raw_message TEXT NOT NULL,
    time INTEGER NOT NULL,
);"#;

static USAGE_TABLE : &str = r#"
CREATE TABLE IF NOT EXISTS usage_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    response_id INTEGER NOT NULL,
    total_tokens INTEGER NOT NULL,
    prompt_tokens INTEGER NOT NULL,
    prompt_cache_hit_tokens INTEGER NOT NULL,
    completion_tokens INTEGER NOT NULL,
    FOREIGN KEY (response_id) REFERENCES response (id)
);"#;

