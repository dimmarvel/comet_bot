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

async fn run(cli: &Client, conf: &Config, t: MsgType) -> Result<Value, Box<dyn std::error::Error>>  {
    let res = send_request(&cli, &conf.tg_token, type_to_str(t), &Default::default()).await?;

    Ok(res)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Client::new();
    let conf = config::load_config("config.json")?;
    println!("{:#?}", conf);
    
    println!("{:#?}", run(&cli, &conf, MsgType::GetMe).await.unwrap());
    println!("{:#?}", run(&cli, &conf, MsgType::GetUpdates).await.unwrap());
    Ok(())
}

