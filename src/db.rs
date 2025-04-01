use sqlx::{SqlitePool,Row};
use std::sync::Arc;
use crate::config::config;
use crate::ll_one_bot::interface::{LLOneBot,SendBack};
use crate::llm_api::interface::{ROLE,Response,Message};
use crate::second2date;
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
    
    sqlx::query(r#"
      INSERT INTO message (self_id, user_id, group_id, time, raw_message)
      VALUES (?, ?, ?, ?, ?)
      ;"#)
      .bind(self_id as i64)
      .bind(user_id as i64)
      .bind(group_id.map(|x| x as i64))
      .bind(time as i64)
      .bind(raw_message)
      .execute(&*self.pool)
      .await?;
    Ok(())
  } 
  
  pub async fn insert_response( //返回自增id
    &self, 
    self_id: u64,
    user_id: Option<u64>, 
    group_id: Option<u64>,
    raw_message: &str, 
    time: u64,
  ) -> Result<u64, sqlx::Error>{
    sqlx::query(r#"
      INSERT INTO response (self_id, user_id, group_id, raw_message, time)
      VALUES (?, ?, ?, ?, ?)
      ;"#,)
      .bind(self_id as i64)
      .bind(user_id.map(|x| x as i64))
      .bind(group_id.map(|x| x as i64))
      .bind(raw_message)
      .bind(time as i64)
      .execute(&*self.pool)
      .await?;

    let row: (i64,) = sqlx::query_as("SELECT last_insert_rowid()")
        .fetch_one(&*self.pool)
        .await?;
    Ok(row.0 as u64)
  }


  pub async fn insert_usage(
    &self, 
    response_id: u64, 
    total_tokens: u64, 
    prompt_tokens: u64, 
    prompt_cache_hit_tokens: u64, 
    completion_tokens: u64
  ) -> Result<(), sqlx::Error>{
    sqlx::query(r#"
      INSERT INTO usage_stats (response_id, total_tokens, prompt_tokens, prompt_cache_hit_tokens, completion_tokens)
      VALUES (?, ?, ?, ?, ?)
      ;"#)
      .bind(response_id as i64)
      .bind(total_tokens as i64)
      .bind(prompt_tokens as i64)
      .bind(prompt_cache_hit_tokens as i64)
      .bind(completion_tokens as i64)
      .execute(&*self.pool)
      .await?;
    Ok(())
  }

  // 返回<qq号，消息，时间>
  pub async fn get_private_context(&self, user_id: u64) -> Result<Vec<(u64, String, u64)>, sqlx::Error> {
    let messages = sqlx::query(
        r#"
        SELECT time, raw_message, user_id AS id
        FROM message
        WHERE user_id = ?
        ORDER BY time DESC
        LIMIT ?
        ;"#)
    .bind(user_id as i64)
    .bind(config::CONTEXT_LIMIT as i32)
    .fetch_all(&*self.pool)
    .await?;
    let responses = sqlx::query(
      r#"
      SELECT time, raw_message, self_id AS id
      FROM response
      WHERE user_id = ?
      ORDER BY time DESC
      LIMIT ?
      ;"#)
    .bind(user_id as i64)
    .bind(config::CONTEXT_LIMIT as i32)
    .fetch_all(&*self.pool)
    .await?;
    let mut combined: Vec<(u64, String, u64)> = Vec::new();  
    for message in messages {
      combined.push((message.get("id"), message.get("raw_message"), message.get("time")));
    }

    for response in responses {
      combined.push((response.get("id"), response.get("raw_message"), response.get("time")));
    }

    combined.sort_by(|a, b| b.2.cmp(&a.2)); // 按时间倒序
    let limited = combined.into_iter().take(config::CONTEXT_LIMIT).collect();
    Ok(limited)
  }

  pub async fn get_group_context(&self, group_id: u64) -> Result<Vec<(u64, String, u64)>, sqlx::Error> {
    let messages = sqlx::query(
        r#"
        SELECT time, raw_message, user_id AS id
        FROM message
        WHERE group_id = ?
        LIMIT ?
        ;"#)
    .bind(group_id as i64)
    .bind(config::CONTEXT_LIMIT as i32)
    .fetch_all(&*self.pool)
    .await?;
    let responses = sqlx::query(
      r#"
      SELECT time, raw_message, self_id AS uid
      FROM response
      WHERE group_id = ?
      LIMIT ?
      ;"#)
    .bind(group_id as i64)
    .bind(config::CONTEXT_LIMIT as i32)
    .fetch_all(&*self.pool)
    .await?;
    let mut combined: Vec<(u64, String, u64)> = Vec::new();  
    for message in messages {
      combined.push((message.get("id"), message.get("raw_message"), message.get("time")));
    }

    for response in responses {
      combined.push((response.get("uid"), response.get("raw_message"), response.get("time")));
    }

    combined.sort_by(|a, b| b.2.cmp(&a.2));
    let limited = combined.into_iter().take(config::CONTEXT_LIMIT).collect();
    Ok(limited)
  }

  pub async fn delete_private_message(&self, user_id: u64) -> Result<(), sqlx::Error> {
    let (result1,result2) = tokio::join!(
    sqlx::query(
        r#"
        DELETE FROM message
        WHERE user_id = ?
        ;"#)
    .bind(user_id as i64)
    .execute(&*self.pool),

    sqlx::query(
      r#"
      DELETE FROM response
      WHERE user_id = ?
      ;"#)
    .bind(user_id as i64)
    .execute(&*self.pool)
    );
    result1?;
    result2?;
    Ok(())
  }

  pub async fn reset_all_table(&self)-> Result<(), sqlx::Error>{
    let (result1,result2,result3) = tokio::join!(
      sqlx::query(
          r#"
          DELETE FROM message
          ;"#)
      .execute(&*self.pool),

      sqlx::query(
        r#"
        DELETE FROM response
        ;"#)
      .execute(&*self.pool),

      sqlx::query(
        r#"
        DELETE FROM usage_stats
        ;"#)
      .execute(&*self.pool)
      );
      result1.unwrap();
      result2.unwrap();
      result3.unwrap();
      Ok(())
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

  pub async fn insert_all(&self, message: &LLOneBot, response: &Response, sendback: &SendBack) -> Result<(), sqlx::Error>{
    let id = self.insert_message_and_sendback(message, sendback).await?;
    self.insert_token_usage(id, response).await?;
    Ok(())
  }

  async fn insert_message_and_sendback(&self, message: &LLOneBot, sendback: &SendBack) ->Result<u64, sqlx::Error>{
    match message{
      LLOneBot::Private(message) =>{
        self.db.insert_message(
          message.self_id,
          message.user_id,
          None,
          message.time,
          message.raw_message.as_str()
        ).await?;
      }
      LLOneBot::Group(message) =>{
        self.db.insert_message(
          message.self_id,
          message.user_id,
          Some(message.group_id),
          message.time,
          message.raw_message.as_str()
        ).await?;
      }
    }
    
    let raw_message: String = sendback.get_content();
    let response_id = match sendback{
      SendBack::Private(sendback) =>{
        self.db.insert_response(
          message.get_self_id(),
          Some(sendback.user_id),
          None,
          raw_message.as_str(),
          message.get_time(),
        ).await?
      },
      SendBack::Group(sendback) =>{
        self.db.insert_response(
          message.get_self_id(),
          None,
          Some(sendback.group_id),
          raw_message.as_str(),
          message.get_time(),
        ).await?
      }
    };
    println!("{}",response_id);
    Ok(response_id)
  }

  async fn insert_token_usage(&self, id: u64, response: &Response) -> Result<(), sqlx::Error>{
    self.db.insert_usage(
      id,
      response.usage.total_tokens,
      response.usage.prompt_tokens,
      response.usage.prompt_cache_hit_tokens,
      response.usage.completion_tokens
      ).await?;
      Ok(())
  }


  pub async fn get_context(&self, message: &LLOneBot) -> Result<Vec<Message>, sqlx::Error>{
    match message{
      LLOneBot::Private(message) =>{
        let context = self.db.get_private_context(message.user_id).await?;
        let mut array = Vec::<Message>::new();
        for i in context.iter().rev(){
          let content = format!("QQ:{},time:{},message:{}", i.0,second2date(i.2 as i64),i.1);
          if i.0 == message.user_id{
            array.push(Message::new(ROLE::User, content));
          }else{
            array.push(Message::new(ROLE::Assistant, i.1.clone()));
          }
        }
        Ok(array)
      },

      LLOneBot::Group(message) =>{
        let context = self.db.get_group_context(message.group_id).await?;
        let mut array = Vec::<Message>::new();
        for i in context.iter().rev(){
          let content = format!("QQ:{},time:{},message:{}", i.0,second2date(i.2 as i64),i.1);
          if i.0 == message.user_id{
            array.push(Message::new(ROLE::User, content));
          }else{
            array.push(Message::new(ROLE::Assistant, i.1.clone()));
          }
        }
        Ok(array)
      }
    }
  }

  pub async fn reset_all_table(&self) -> Result<(), sqlx::Error>{
    self.db.reset_all_table().await?;
    Ok(())
  }
}




static MESSAGE_TABLE : &str = r#"
CREATE TABLE IF NOT EXISTS message (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    self_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    group_id INTEGER,
    time INTEGER NOT NULL,
    raw_message TEXT NOT NULL
);"#;

static RESPONSE_TABLE : &str = r#"
CREATE TABLE IF NOT EXISTS response (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    self_id INTEGER NOT NULL,
    user_id INTEGER,
    group_id INTEGER,
    raw_message TEXT NOT NULL,
    time INTEGER NOT NULL
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

