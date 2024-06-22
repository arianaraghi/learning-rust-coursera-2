//! This is a part of week-2, separated due to complexity.
//! This is not working for me due to lack of API Key for OpenAI. 
//! This is because I am poor and have not enough money to have a subscription. :D

use chatbot::chatbot::run_chat_loop;
use reqwest::Client;

use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();

    // Use env variable OPENAI_API_KEY
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let url = "https://api.openai.com/v1/completions";

    run_chat_loop(&client, &api_key, url).await?;

    Ok(())
}