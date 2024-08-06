use std::collections::HashMap;
use reqwest::Client;
use serde_json::Value;

pub enum MsgType {
    GetMe,
    GetUpdates,
    SendMessage,
}

pub fn type_to_str(t: &MsgType) -> &'static str {
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
    params: &HashMap<&str, &str>,
) -> Result<serde_json::Value, reqwest::Error> {
    let mut url = String::new();
    url.push_str("https://api.telegram.org/bot");
    url.push_str(api_token);
    url.push_str("/");
    url.push_str(method);

    let response = client.get(&url).query(params).send().await?;
    let json: Value = response.json().await?;
    Ok(json)
}
