pub use actix_web::Resource as Factory;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::config::enabled_true;

pub const ROUTE: &str = "/";

#[derive(Debug, Default, Deserialize, Clone)]
pub struct WWWConfig {
    #[serde(default = "enabled_true")]
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct WWWService {
    config: WWWConfig,
}

impl WWWService {
    pub fn new(config: WWWConfig) -> WWWService {
        WWWService { config }
    }

    async fn homepage() -> impl Responder {
        HttpResponse::Ok().body("Coming soon!")
    }

    pub fn factory(&self) -> Factory {
        web::resource(ROUTE).route(web::get().to(WWWService::homepage))
    }
}
