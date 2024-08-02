use reqwest::Client;
mod config;
mod tg_bot;

pub use config::Config;
pub use std::error::Error;
pub use tg_bot::send_request;

async fn tmp(conf: &Config) {
    let client = Client::new();
    match send_request(&client, &conf.tg_token, "getMe", &Default::default()).await {
        Ok(json) => println!("Response JSON: {:?}", json),
        Err(e) => println!("Error: {:?}", e),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conf = config::load_config("config.json")?;
    println!("{:#?}", conf);
    
    tmp(&conf).await;

    Ok(())
}

