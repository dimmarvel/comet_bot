use log::debug;
use reqwest::Client;
mod config;
mod tg_bot;
mod args;
mod tg_objects;

pub use config::Config;
pub use std::error::Error;
pub use tg_bot::MsgType;
pub use tg_bot::run;

use clap::Parser;
use env_logger;
use args::Verbose;
use args::Arguments;
use std::str::FromStr;

pub struct Application {
    pub cli: Client,
    pub conf: Config,
    pub args: Arguments,
    pub log_level: &'static str,
}

impl Application {
    pub fn init() -> Result<Self, Box<dyn Error>> {
        let cli = Client::new();
        let conf = config::load_config("config.json")?;
        let args = args::Arguments::parse();

        let arg_line = std::env::args().skip(1).map(|arg| arg.to_string()).collect::<Vec<String>>().join(" ");

        debug!("Args: {}", arg_line);

        let log_level = match args.verbose {
            Verbose::Debug => "debug",
            Verbose::Info => "info",
            Verbose::Warn => "warn",
            Verbose::Error => "error",
        };

        env_logger::Builder::new()
        .filter_level(log::LevelFilter::from_str(log_level).unwrap())
        .init();

        Ok(Application { cli, conf, args, log_level })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Application::init()?;
    
    run(&app.cli, &app.conf, &MsgType::GetUpdates).await;
    Ok(())
}

