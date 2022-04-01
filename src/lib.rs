use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServerConfig {
    pub network: NetworkConfig,
    pub params: ParamsConfig,
}

impl ServerConfig {
    pub fn load() -> Self {
        if let Ok(content) = fs::read_to_string("fixtures/config.yml").await {
            serde_yaml::from_str(&content).unwrap()
        } else {
            Self::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub host: String,
    pub port: u16,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".into(),
            port: 3000,
        }
    }
}

impl From<NetworkConfig> for SocketAddr {
    fn from(config: NetworkConfig) -> Self {
        format!("{}:{}", config.host, config.port).parse().unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamsConfig {
    pub max_size: u32,
    pub min_size: u32,
}

impl Default for ParamsConfig {
    fn default() -> Self {
        Self {
            max_size: 10,
            min_size: 3,
        }
    }
}
