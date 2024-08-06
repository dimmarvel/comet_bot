use reqwest::Client;
mod config;
mod tg_bot;
mod tg_objects;

pub use config::Config;
pub use serde_json::Value;
pub use std::error::Error;
pub use tg_bot::send_request;
pub use tg_bot::type_to_str;
pub use tg_bot::MsgType;

use std::collections::HashMap;

async fn run(cli: &Client, conf: &Config, t: &MsgType) {
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
            &Default::default()).await;
    
        // Check if there are any updates
        if let Ok(response) = response {
            if let Some(updates) = response["result"].as_array() {
                // Process each update
                for update in updates {
                    offset = update["update_id"].as_i64().unwrap() + 1;

                    println!("{}", offset);
                }
            }
        }
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Client::new();
    let conf = config::load_config("config.json")?;
    println!("{:#?}", conf);
    run(&cli, &conf, &MsgType::GetMe).await;
    Ok(())
}

