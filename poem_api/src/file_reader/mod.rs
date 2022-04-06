
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub redis_host: String,
    pub redis_port: i16,
    pub mongo_host: String,
    pub mongo_port: i16,
    pub mongo_db_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirebaseConfig {
    pub api_key: String,
}

pub fn get_config() -> Config {
    let config_file = "../Config/config.json";
    let config_file_content = std::fs::read_to_string(config_file).unwrap();
    let config: Config = serde_json::from_str(&config_file_content).unwrap();
    return config;
}

pub fn get_firebase_config() -> FirebaseConfig {
    let config_file = "../FirebaseConfig/config.json";
    let config_file_content = std::fs::read_to_string(config_file).unwrap();
    let config: FirebaseConfig = serde_json::from_str(&config_file_content).unwrap();
    return config;
}
