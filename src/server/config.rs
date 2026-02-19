use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub listeners: Vec<ListenerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerConfig {
    pub port: u16,
    pub protocol: String,
    pub bind_addr: String,
}

impl ServerConfig {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn example() -> Self {
        Self {
            listeners: vec![
                ListenerConfig {
                    port: 8080,
                    protocol: "ProtocolA".to_string(),
                    bind_addr: "0.0.0.0".to_string(),
                },
                ListenerConfig {
                    port: 8081,
                    protocol: "ProtocolB".to_string(),
                    bind_addr: "0.0.0.0".to_string(),
                },
            ],
        }
    }
}