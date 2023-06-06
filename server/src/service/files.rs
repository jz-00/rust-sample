pub use actix_files::Files as Factory;
use serde::Deserialize;

use crate::config::enabled_true;

pub const ROUTE: &str = "/static";

#[derive(Debug, Default, Deserialize, Clone)]
pub struct FilesConfig {
    #[serde(default = "enabled_true")]
    pub enabled: bool,
    pub static_dir: String,
}

#[derive(Debug, Clone)]
pub struct FilesService {
    config: FilesConfig,
}

impl FilesService {
    pub fn new(config: FilesConfig) -> FilesService {
        FilesService { config }
    }

    pub fn factory(&self) -> Factory {
        actix_files::Files::new(ROUTE, &self.config.static_dir)
    }
}
