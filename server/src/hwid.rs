use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Serialize, Deserialize)]
pub struct HwidList {
    pub valid_hwids: Vec<String>,
}

impl HwidList {
    pub fn load() -> Self {
        let data = fs::read_to_string("hwids.yaml")
            .expect("Unable to read hwids.yaml");
        serde_yaml::from_str(&data)
            .expect("Unable to parse YAML")
    }

    pub fn is_valid(&self, hwid: &str) -> bool {
        self.valid_hwids.contains(&hwid.to_string())
    }
}