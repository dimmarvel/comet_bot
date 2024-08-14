mod config;
mod tg_bot;
mod args;
mod tg_objects;
mod tg_handlers;
mod tg_utils;
mod application;

pub use config::Config;
pub use std::error::Error;
pub use tg_utils::MsgType;
pub use tg_bot::run;
pub use application::Application;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Application::init()?;
    
    run(app, &MsgType::GetUpdates).await;
    Ok(())
}

