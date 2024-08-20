use crate::args;
use crate::config;
use config::Config;
use clap::Parser;
use args::Verbose;
use args::Arguments;
use log::debug;
use reqwest::Client;
use std::str::FromStr;
use std::error::Error;
use env_logger;

#[derive(Debug, Clone)]
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