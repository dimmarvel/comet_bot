use std::collections::HashMap;
use reqwest::Client;
use serde_json::Value;
use reqwest::Error;

pub async fn send_request(
    client: &Client,
    api_token: &str,
    method: &str,
    params: &HashMap<&str, &str>,
) -> Result<Value, Error> {
    let mut url = String::new();
    url.push_str("https://api.telegram.org/bot");
    url.push_str(api_token);
    url.push_str("/");
    url.push_str(method);

    let response = client.get(&url).query(params).send().await?;
    let json: Value = response.json().await?;
    Ok(json)
}
