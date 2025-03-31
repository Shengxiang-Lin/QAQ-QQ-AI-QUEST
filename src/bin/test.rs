use QAQ::{ll_one_bot::interface::SendBackIntermediate, llm_api::interface::*, db::Database};
use QAQ::{API_SENDER, QQ_SENDER};
use tokio::task::LocalSet;
use dotenv::dotenv;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database at {}", url);
    let database = Database::new(&url).await?;
    Ok(())
}