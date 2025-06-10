use sqlx::{SqlitePool,Row};
use std::sync::Arc;
use crate::config::get_config;
use crate::ll_one_bot::interface::{LLOneBot,SendBack};
use crate::llm_api::interface::{ROLE,Response,Message};
use actix_web::{HttpResponse};
use crate::{second2date};

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
    .bind(get_config().context_limit as i32)
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
    .bind(get_config().context_limit as i32)
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
    let limited = combined.into_iter().take(get_config().context_limit).collect();
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
    .bind(get_config().context_limit as i32)
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
    .bind(get_config().context_limit as i32)
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
    let limited = combined.into_iter().take(get_config().context_limit).collect();
    Ok(limited)
  }

  pub async fn delete_private_message(&self, user_id: u64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        SELECT self_id FROM message
        WHERE user_id = ? AND group_id IS NULL
        ORDER BY time DESC
        LIMIT 1
        ;"#)
    .bind(user_id as i64)
    .fetch_one(&*self.pool)
    .await?
    .get::<u64, _>(0);


    let (result1,result2) = tokio::join!(
    sqlx::query(
        r#"
        DELETE FROM message
        WHERE user_id = ? AND group_id IS NULL
        ;"#)
    .bind(user_id as i64)
    .execute(&*self.pool),

    sqlx::query(
      r#"
      DELETE FROM response
      WHERE user_id = ? AND group_id IS NULL
      ;"#)
    .bind(user_id as i64)
    .execute(&*self.pool)
    );
    result1?;
    result2?;
    Ok(result)
  }

  pub async fn delete_group_message(&self, group_id: u64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
      r#"
      SELECT self_id FROM message
      WHERE group_id = ?
      ORDER BY time DESC
      LIMIT 1
      ;"#)
  .bind(group_id as i64)
  .fetch_one(&*self.pool)
  .await?
  .get::<u64, _>(0);
    
    let (result1,result2) = tokio::join!(
      sqlx::query(
          r#"
          DELETE FROM message
          WHERE group_id = ?
          ;"#)
      .bind(group_id as i64)
      .execute(&*self.pool),

      sqlx::query(
        r#"
        DELETE FROM response
        WHERE group_id = ?
        ;"#)
      .bind(group_id as i64)
      .execute(&*self.pool)
    );
    result1?;
    result2?;
    Ok(result)
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

  pub async fn get_private_context_count(&self, user_id: u64) -> Result<usize, sqlx::Error> {
    let mut count = sqlx::query(
        r#"
        SELECT COUNT(*)
        FROM message
        WHERE user_id = ?
        ;"#)
    .bind(user_id as i64)
    .fetch_one(&*self.pool)
    .await?
    .get::<i64, _>(0);
    
    count += sqlx::query(
      r#"
      SELECT COUNT(*)
      FROM response
      WHERE user_id = ?
      ;"#)
    .bind(user_id as i64)
    .fetch_one(&*self.pool)
    .await?
    .get::<i64, _>(0);
    Ok(count as usize)
  }

  pub async fn get_group_context_count(&self, group_id: u64) -> Result<usize, sqlx::Error> {
    let mut count = sqlx::query(
        r#"
        SELECT COUNT(*)
        FROM message
        WHERE group_id = ?
        ;"#)
    .bind(group_id as i64)
    .fetch_one(&*self.pool)
    .await?
    .get::<i64, _>(0);
    
    count += sqlx::query(
      r#"
      SELECT COUNT(*)
      FROM response
      WHERE group_id = ?
      ;"#)
    .bind(group_id as i64)
    .fetch_one(&*self.pool)
    .await?
    .get::<i64, _>(0);
    Ok(count as usize)
  }
}

pub struct DatabaseManager{
  pub db: Database,
  //预留缓存

}


impl Clone for DatabaseManager {
  fn clone(&self) -> Self {
      DatabaseManager {
          db: Database {
              pool: Arc::clone(&self.db.pool),
          }
      }
  }
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
    let user_id = message.get_user_id();
    let group_id = match message {
      LLOneBot::Private(message) => None,
      LLOneBot::Group(message) => Some(message.group_id),
    };
    let db_manager = Arc::new(self.clone());
    tokio::spawn(async move {
      match db_manager.integrate_context(user_id, group_id).await {
          Ok(_) => { /* 操作成功 */ },
          Err(e) => eprintln!("Error integrating context: {:?}", e),
      }
  });
    Ok(())
  }


  pub async fn insert_integrate_context(&self, response: &Response, self_id:u64, user_id: u64, group_id: Option<u64>) -> Result<(), sqlx::Error>{
    let id = self.db.insert_response(
      self_id,
      Some(user_id),
      group_id,
      response.get_content().as_str(),
      response.created,
    ).await?;
    self.insert_token_usage(id, response).await?;
    println!("Insert integrate context success, id: {}", id);
    Ok(())
  }


  pub async fn integrate_context(&self, user_id: u64, group_id: Option<u64>) -> Result<(), sqlx::Error>{
    let count = if let Some(group_id) = group_id {
      self.db.get_group_context_count(group_id).await?
    } else {
      self.db.get_private_context_count(user_id).await?
    };
    if count >= get_config().context_limit - 2 {
      if let Err(e) = self.update_context(user_id, group_id).await{
        eprintln!("Error updating context: {:?}", e);
      }
    } else {
      println!("Context not enough, no need to update");
    }
    Ok(())
  }


  async fn update_context(&self,user_id: u64, group_id: Option<u64>) -> Result<(), sqlx::Error>{
    use crate::llm_api::interface::DeepSeek;
    use crate::config::{get_config,model_url};
    use crate::{SELECTED_MODEL,API_SENDER};
    let model_name = {
      let selected_model = SELECTED_MODEL.lock().unwrap();
      selected_model.clone()
    };
    let mut request = DeepSeek::new(model_name, Some(get_config().presence_penalty), Some(get_config().temperature));
    request.replace_by_integrate_prompt();
    if let Some(group_id) = group_id{
      let context = self.db.get_group_context(group_id).await.unwrap();
      let mut array = Vec::<Message>::new();
      for i in context.iter().rev(){
        let content = format!("QQ:{},time:{},message:{}", i.0,second2date(i.2 as i64),i.1);
        if i.0 == user_id{
          array.push(Message::new_text(ROLE::User, content));
        }else{
          array.push(Message::new_text(ROLE::Assistant, i.1.clone()));
        }
      }
      request.extend_message(array);
    }
    else{
      let context = self.db.get_private_context(user_id).await.unwrap();
      let mut array = Vec::<Message>::new();
      for i in context.iter().rev(){
        let content = format!("QQ:{},time:{},message:{}", i.0,second2date(i.2 as i64),i.1);
        if i.0 == user_id{
          array.push(Message::new_text(ROLE::User, content));
        }else{
          array.push(Message::new_text(ROLE::Assistant, i.1.clone()));
        }
      }
      request.extend_message(array);
    }
    request.add_message(Message::new_text(ROLE::User, "现在请总结上下文".to_string()));
    println!("Request: {:?}", request);
    let result:Result<Response,Box<dyn std::error::Error + Send+ Sync>> = match request.model.as_str(){
      "doubao-1.5-vision-pro-32k-250115" => API_SENDER.send_api_post(model_url::DOUBAO_VISION,&request).await,      
      "deepseek-chat" => API_SENDER.send_api_post(model_url::DEEPSEEK,&request).await,
      _ => panic!("Invalid model name: {}", request.model),
    };
    if let Ok(response) = result{
      let self_id = match group_id{
        Some(group_id) => self.db.delete_group_message(group_id).await.unwrap(),
        None => self.db.delete_private_message(user_id).await.unwrap(),
      };
      self.insert_integrate_context(&response, self_id, user_id, group_id).await.unwrap();
    }else{
      eprintln!("AN ERROR OCCUR:{:?}",result); 
    }
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
          Some(sendback.user_id),
          Some(sendback.group_id),
          raw_message.as_str(),
          message.get_time(),
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
      response.usage.prompt_tokens_details.cached_tokens,
      response.usage.completion_tokens
      ).await?;
      Ok(())
  }


  pub async fn get_context(&self, message: &LLOneBot) -> Result<Vec<Message>, sqlx::Error>{
    let context = match message{
      LLOneBot::Private(message) =>{
        self.db.get_private_context(message.user_id).await.unwrap()
      },
      LLOneBot::Group(message) =>{
        self.db.get_group_context(message.group_id).await.unwrap()
      }
    };
    let user_id = message.get_user_id();
    let mut array = Vec::<Message>::new();
    for i in context.iter().rev(){
      let content = format!("QQ:{},time:{},message:{}", i.0,second2date(i.2 as i64),i.1);
      if i.0 == user_id{
        array.push(Message::new_text(ROLE::User, content));
      }else{
        array.push(Message::new_text(ROLE::Assistant, i.1.clone()));
      }
    }
    Ok(array)
  }

  pub async fn reset_all_table(&self) -> Result<(), sqlx::Error>{
    self.db.reset_all_table().await?;
    Ok(())
  }

  pub async fn generate_test_data(&self) -> Result<(), sqlx::Error> {
    // 构造测试用的 QQ 号和群号
    let self_id = 12345678;  // 机器人 QQ 号
    let user_ids = [987654321, 876543210, 765432109];  // 测试用户 QQ 号
    let group_ids = [123456789, 234567890];  // 测试群号
    
    // 当前时间戳（秒）
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    println!("开始生成测试数据...");
    
    // 1. 生成私聊消息
    println!("正在生成私聊消息数据...");
    for (idx, &user_id) in user_ids.iter().enumerate() {
        // 每个用户生成多条对话
        for i in 0..5 {
            // 用户发送的消息
            let user_message = format!("这是用户 {} 发送的第 {} 条测试消息", user_id, i + 1);
            let msg_time = now - (10 * (5 - i)) - idx as u64 * 100;  // 时间依次递增
            
            self.db.insert_message(
                self_id,
                user_id,
                None,  // 私聊没有群号
                msg_time,
                user_message.as_str()
            ).await?;
            
            // 机器人的回复
            let bot_reply = format!("这是机器人回复用户 {} 的第 {} 条消息", user_id, i + 1);
            let reply_time = msg_time + 2;  // 回复晚 2 秒
            
            let response_id = self.db.insert_response(
                self_id,
                Some(user_id),
                None,  // 私聊没有群号
                bot_reply.as_str(),
                reply_time
            ).await?;
            
            // 生成使用统计数据
            self.db.insert_usage(
                response_id,
                100 + i * 10,  // total_tokens
                70 + i * 5,    // prompt_tokens
                10,            // prompt_cache_hit_tokens
                30 + i * 5     // completion_tokens
            ).await?;
        }
        println!("用户 {} 的私聊数据生成完成", user_id);
    }
    
    // 2. 生成群聊消息
    println!("正在生成群聊消息数据...");
    for (group_idx, &group_id) in group_ids.iter().enumerate() {
        // 在每个群中，让不同用户发言
        for (user_idx, &user_id) in user_ids.iter().enumerate().take(2) {  // 每个群只让前两个用户发言
            // 每个用户在每个群发送多条消息
            for i in 0..3 {
                // 用户在群里发送的消息
                let group_msg = format!("这是用户 {} 在群 {} 发送的第 {} 条测试消息", 
                                     user_id, group_id, i + 1);
                let msg_time = now - (10 * (3 - i)) - group_idx as u64 * 50 - user_idx as u64 * 20;
                
                self.db.insert_message(
                    self_id,
                    user_id,
                    Some(group_id),
                    msg_time,
                    group_msg.as_str()
                ).await?;
                
                // 如果是第一个用户的消息，机器人回复
                if user_idx == 0 {
                    let bot_reply = format!("这是机器人在群 {} 回复用户 {} 的第 {} 条消息", 
                                         group_id, user_id, i + 1);
                    let reply_time = msg_time + 3;  // 回复晚 3 秒
                    
                    let response_id = self.db.insert_response(
                        self_id,
                        Some(user_id),
                        Some(group_id),
                        bot_reply.as_str(),
                        reply_time
                    ).await?;
                    
                    // 生成使用统计数据
                    self.db.insert_usage(
                        response_id,
                        150 + i * 15,  // total_tokens
                        100 + i * 10,  // prompt_tokens
                        15,            // prompt_cache_hit_tokens
                        50 + i * 5     // completion_tokens
                    ).await?;
                }
            }
            println!("用户 {} 在群 {} 的消息数据生成完成", user_id, group_id);
        }
    }
    
    println!("测试数据生成完成！");
    println!("- 生成了 {} 个用户的私聊数据", user_ids.len());
    println!("- 生成了 {} 个群聊的数据", group_ids.len());
    
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
    completion_tokens INTEGER NOT NULL
);"#;