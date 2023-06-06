pub use actix_web::Resource as Factory;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde::Deserialize;

use super::juniper_actix;
use crate::config::enabled_true;
use crate::util::authenticate_user;
use graphql::{schema, ContextManager, Schema};

// GraphQL
pub const ROUTE: &str = "/graphql";

#[derive(Debug, Default, Deserialize, Clone)]
pub struct GraphQLConfig {
    #[serde(default = "enabled_true")]
    pub enabled: bool,
}

#[derive(Clone)]
pub struct GraphQLService {
    config: GraphQLConfig,
    context_manager: web::Data<ContextManager>,
}

impl std::fmt::Debug for GraphQLService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphQLService").finish()
    }
}

impl GraphQLService {
    pub async fn new(config: GraphQLConfig) -> GraphQLService {
        GraphQLService {
            config,
            context_manager: web::Data::new(ContextManager::new().await),
        }
    }

    async fn graphql_handler(
        req: HttpRequest,
        payload: web::Payload,
        schema: web::Data<Schema>,
        context_manager: web::Data<ContextManager>,
    ) -> Result<HttpResponse> {
        let user = authenticate_user(&req)?;
        let context = context_manager.create_user_context(user);
        juniper_actix::graphql_handler(&schema, &context, req, payload).await
    }

    pub fn factory(&self) -> Factory {
        web::resource(ROUTE)
            .app_data(web::Data::new(schema::create()))
            .app_data(self.context_manager.clone())
            .route(web::post().to(GraphQLService::graphql_handler))
            .route(web::get().to(GraphQLService::graphql_handler))
    }
}
