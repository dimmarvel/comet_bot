

pub use crate::config::Config;
use std::collections::HashMap;
use reqwest::Client;
use serde_json::Value;
use log;

pub enum MsgType {
    GetMe,
    GetUpdates,
    SendMessage,
}

pub fn type_to_str(t: &MsgType) -> &'static str 
{
    match t {
        MsgType::GetMe => "getMe",
        MsgType::GetUpdates => "getUpdates",
        MsgType::SendMessage => "sendMessage",
    }
}

pub async fn send_request(
    client: &Client,
    api_token: &str,
    method: &str,
    params: &HashMap<&str, String>,
) -> Result<serde_json::Value, reqwest::Error> 
{
    let mut url = String::new();
    url.push_str("https://api.telegram.org/bot");
    url.push_str(api_token);
    url.push_str("/");
    url.push_str(method);

    let response = client.get(&url).query(params).send().await?;
    let json: Value = response.json().await?;
    Ok(json)
}

async fn handle_message(updates: &Vec<Value>, offset: &mut i64, cli: &Client, conf: &Config) -> Result<(), Box<dyn std::error::Error>>
{
    // Process each update
    for update in updates {
        println!("{:#?}", update);
        if let Some(message) = update["message"].as_object() {
            log::debug!("{:#?}", message);
            let from = message["from"].as_object().unwrap();
            let chat = message["chat"].as_object().unwrap();
            let chat_id = chat["id"].as_i64().unwrap();
            let text = message["text"].as_str().unwrap();

            let mut params: HashMap<&str, String> = HashMap::new();
            params.insert("chat_id", chat_id.to_string());
            params.insert("text", text.to_string());

            let _response = send_request(cli, &conf.tg_token, "sendMessage", &params).await?;

            *offset = update["update_id"].as_i64().unwrap() + 1;
        }
    }
    Ok(())
}



pub async fn run(cli: &Client, conf: &Config, t: &MsgType)
{
    // Set the initial offset to 0
    let mut offset: i64 = 0;
    loop {
        // Set up the parameters for the getUpdates method
        let mut params = HashMap::new();
        params.insert("offset", offset.to_string());
        params.insert("timeout", "2".to_string());
    
        // Send the request and get the response
        let response = send_request(
            &cli, &conf.tg_token, 
            type_to_str(t), 
            &params).await;
    
        // Check if there are any updates
        if let Ok(response) = response {
            if let Some(updates) = response["result"].as_array() {
                match handle_message(updates, &mut offset, cli, conf).await {
                    Ok(_) => println!("Message handled successfully"),
                    Err(e) => eprintln!("Error handling message: {}", e),
                };
            }
            else {
                println!("Something wrong {:#?} ", response);
            }
        }
        else {
            println!("Error response {}", offset);
        }
    }

}