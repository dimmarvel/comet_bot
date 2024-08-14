use std::fs::File;
use serde::{Deserialize, Serialize};
use std::io::{self, Read, ErrorKind};
use serde_json;

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct Config {
    pub tg_token: String,
    pub ip_address: String,
}

pub fn load_config(file_path: &str) -> Result<Config, io::Error> {
    let file = File::open(file_path);
    if file.is_err() {
        eprintln!("Error opening file {}: {}", file_path, file.unwrap_err());
        return Err(ErrorKind::NotFound.into());
    }
    let mut file = file.unwrap();

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        eprintln!("Error reading file {}: {}", file_path, contents.trim());
        return Err(ErrorKind::InvalidInput.into());
    }

    serde_json::from_str(&contents).map_err(|e| {
        eprintln!("Error parsing JSON in file {}: {}", file_path, e);
        ErrorKind::InvalidInput.into()
    })
}