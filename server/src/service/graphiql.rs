pub use actix_web::Resource as Factory;
use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;

use super::graphql;
use super::juniper_actix;
use crate::config::enabled_true;

pub const ROUTE: &str = "/graphiql";

#[derive(Debug, Default, Deserialize, Clone)]
pub struct GraphiQLConfig {
    #[serde(default = "enabled_true")]
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct GraphiQLService {
    config: GraphiQLConfig,
}

impl GraphiQLService {
    pub fn new(config: GraphiQLConfig) -> GraphiQLService {
        GraphiQLService { config }
    }

    async fn graphiql_handler() -> Result<HttpResponse> {
        // juniper_actix::graphiql_handler(GRAPHQL_ENDPOINT, None).await
        juniper_actix::playground_handler(graphql::ROUTE, None).await
    }

    pub fn factory(&self) -> Factory {
        web::resource(ROUTE).route(web::get().to(GraphiQLService::graphiql_handler))
    }
}
