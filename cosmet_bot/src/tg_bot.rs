
use crate::config::Config;
use crate::tg_utils::{MsgType, type_to_str};
use crate::tg_handlers::handle_message;
use std::collections::HashMap;
use reqwest::Client;
use serde_json::Value;
use log::{debug, error};

pub async fn send_request(
    client: &Client,
    api_token: &str,
    method: &str,
    params: &HashMap<&str, String>,
) -> Result<serde_json::Value, reqwest::Error> 
{
    debug!("{:?}", params);
    let mut url = String::new();
    url.push_str("https://api.telegram.org/bot");
    url.push_str(api_token);
    url.push_str("/");
    url.push_str(method);

    let response = client.get(&url).query(params).send().await?;
    let json: Value = response.json().await?;
    Ok(json)
}

pub async fn run(cli: &Client, conf: &Config, t: &MsgType)
{
    // Set the initial offset to 0
    let mut offset: i64 = 0;
    loop {
        // Set up the parameters for the getUpdates method
        let mut params = HashMap::new();
        params.insert("offset", offset.to_string());
        params.insert("timeout", "30".to_string());
    
        // Send the request and get the response
        let response = send_request(
            &cli, &conf.tg_token, 
            type_to_str(t), 
            &params).await;
    
        // Check if there are any updates
        if let Ok(response) = response {
            if let Some(response_res) = response["result"].as_array() {
                let _ = match handle_message(response_res, &mut offset, cli, conf).await {
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