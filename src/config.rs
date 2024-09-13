use serde::Deserialize;
use tokio::fs;

#[derive(Deserialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub settings: Option<SettingsConfig>,  // Optional for future configurations
}

#[derive(Deserialize)]
pub struct NetworkConfig {
    pub interfaces: Vec<String>,
}

#[derive(Deserialize)]
pub struct SettingsConfig {
    pub log_level: Option<String>,
    pub storage_path: Option<String>,
}

// Function to load and parse the configuration file
pub async fn load_config(file_path: &str) -> Config {
    let config_data = fs::read_to_string(file_path)
        .await
        .expect("Failed to read custom config file.");
    serde_json::from_str(&config_data)
        .expect("Failed to parse custom config.")
}
