use std::fs::File;
use std::io::BufReader;

use anyhow::Context;
use serde::Deserialize;

use crate::network::NetworkConfig;
use crate::service::manager::ServicesConfig;

pub fn get_env_var_or_default(key: &str, default: &str) -> String {
    let default = default.to_owned();
    dotenv::var(key).unwrap_or(default)
}

pub fn enabled_true() -> bool {
    true
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(default)]
    pub network: NetworkConfig,
    pub services: ServicesConfig,
}

impl ServerConfig {
    pub fn load() -> anyhow::Result<ServerConfig> {
        let config_filename = get_env_var_or_default("CONFIG_FILE", "");

        if config_filename.is_empty() {
            error!("CONFIG_FILE env var not set");
            Ok(ServerConfig::default())
        } else {
            let context = || format!("server config file '{}'", &config_filename);
            let reader = BufReader::new(File::open(&config_filename).with_context(context)?);
            Ok(serde_json::from_reader(reader).with_context(context)?)
        }
    }
}
