use serde::{Deserialize, Serialize};
use std::fs;
use rand::Rng;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WebhookEmbedConfig {
    pub title_template: String,
    pub description_template: String,
    pub color_success: u32,
    pub color_failure: u32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub webhook_url: String,
    pub api_key: String,
    pub port: u16,
    pub webhook_embed: WebhookEmbedConfig
}

impl Config {
    pub fn load() -> Self {
        let data = fs::read_to_string("config.yaml")
            .expect("Unable to read config.yaml");
        let mut config: Config = serde_yaml::from_str(&data)
            .expect("Unable to parse YAML");

        if config.port == 0 {
            let mut rng = rand::thread_rng();
            config.port = rng.gen_range(1024..65535);
        }

        config
    }
}