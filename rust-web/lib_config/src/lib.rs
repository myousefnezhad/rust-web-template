use dotenv::dotenv;
use log::*;
use serde::{Deserialize, Serialize};
use std::{env, fs};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub bind_backend: String, // 0.0.0.0:9000
    pub workers_backend: usize,
    pub api_version: String,        // v1
    pub log_level: String,          // Debug, Info, Warn, Error
    pub pg_connection: usize,
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv().ok();
        match env::var("APP_CONFIG") {
            Err(e) => {
                debug!("{}", e);
                panic!("Cannot locate config file; please set APP_CONFIG env variable");
            }
            Ok(config_file_path) => match fs::File::open(config_file_path) {
                Err(e) => {
                    debug!("{}", e);
                    panic!("Cannot read config file");
                }
                Ok(config_file) => match serde_json::from_reader(config_file) {
                    Err(e) => {
                        debug!("{}", e);
                        panic!("Cannot parse json");
                    }
                    Ok(json) => return json,
                },
            },
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_app_config() {
        let config = AppConfig::new();
        println!("{:#?}", config);
    }
}
