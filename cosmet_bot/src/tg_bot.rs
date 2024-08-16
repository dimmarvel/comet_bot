use crate::application::Application;
use crate::tg_utils::{MsgType, msg_type_to_str};
use crate::tg_handlers::handle_message;
use crate::tg_objects::Message;
use std::collections::HashMap;
use reqwest::Client;
use serde_json::Value;
use log::{debug, error};

pub struct MsgRequest{
    pub app: Application,
    pub update_id: i64,
    pub method: MsgType,
    pub msg: Message,
}

impl MsgRequest {
    pub fn new(app: Application, update_id: i64, method: MsgType, msg: Message) -> Self {
        MsgRequest {app, update_id, method, msg }
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


pub async fn send_msg(
    offset: &mut i64,
    msg : &mut MsgRequest
) -> Result<serde_json::Value, reqwest::Error> 
{
    let mut params: HashMap<&str, String> = HashMap::new();
    params.insert("chat_id", msg.msg.chat.id.to_string());
    params.insert("text", format!("{}: {}", msg.msg.from.first_name, msg.msg.text));
    debug!("Send message: {:?}", params);
    let _response = send_request(&msg.app.cli, &msg.app.conf.tg_token, msg_type_to_str(&msg.method), &params).await?;
    
    *offset = msg.update_id + 1;
    Ok(_response)
}

pub async fn run(app : Application, t: &MsgType)
{
    // Set the initial offset to 0
    let mut offset: i64 = 0;
    loop {
        // Set up the parameters for the getUpdates method
        let mut params = HashMap::new();
        params.insert("offset", offset.to_string());
        params.insert("timeout", "1".to_string());
    
        // Send the request and get the response
        let response = send_request(
            &app.cli, &app.conf.tg_token, 
            msg_type_to_str(t), 
            &params).await;
    
        // Check if there are any updates
        if let Ok(response) = response {
            if let Some(response_res) = response["result"].as_array() {
                let _ = match handle_message(app.clone(), response_res, &mut offset).await {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                };
            }
            else {
                error!("Message have no result {:#?} ", response);
            }
        }
        else {
            error!("Response {}", offset);
        }
    }

}