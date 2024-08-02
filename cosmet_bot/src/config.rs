use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub tg_token: String,
    pub ip_address: String,
}

pub fn load_config(file_path: &str) -> Result<Config, Error> {
    let mut file = File::open(file_path).map_err(Error::io)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(Error::io)?;
    serde_json::from_str(&contents)
}