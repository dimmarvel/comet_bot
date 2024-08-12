use crate::tg_objects::Message;
use crate::config::Config;
use crate::tg_bot::send_request;
use serde_json::Value;
use reqwest::Client;
use std::collections::HashMap;
use log::error;

pub async fn handle_message(response_results: &Vec<Value>, offset: &mut i64, cli: &Client, conf: &Config) -> Result<(), Box<dyn std::error::Error>>
{
    // Process each update
    for res in response_results {
        if res.get("message").is_some() && res["message"].is_object() {
            let msg_obj: Value = serde_json::from_str(res["message"].to_string().as_str()).unwrap();
            let msg: Message = serde_json::from_value(msg_obj).unwrap();
    
            let mut params: HashMap<&str, String> = HashMap::new();
            params.insert("chat_id", msg.chat.id.to_string());
            params.insert("text", msg.from.first_name + ": " + &msg.text.to_string());
    
            let _response = send_request(cli, &conf.tg_token, "sendMessage", &params).await?;
    
            *offset = res["update_id"].as_i64().unwrap() + 1;
        }
        else {
            error!("Message have no message json object{:?}", res);
        }
    }
    Ok(())
}
