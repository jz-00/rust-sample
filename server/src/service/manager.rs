use actix_service::ServiceFactory;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::Error;
use actix_web::App;
use serde::Deserialize;

use super::files::{FilesConfig, FilesService};
use super::graphiql::{GraphiQLConfig, GraphiQLService};
use super::graphql::{GraphQLConfig, GraphQLService};
use super::upload::{UploadConfig, UploadService};
use super::www::{WWWConfig, WWWService};

#[derive(Debug, Deserialize, Clone)]
pub enum ServiceName {
    Files,
    GraphiQL,
    GraphQL,
    Upload,
    WWW,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct ServicesConfig {
    pub files: Option<FilesConfig>,
    pub graphiql: Option<GraphiQLConfig>,
    pub graphql: Option<GraphQLConfig>,
    pub upload: Option<UploadConfig>,
    pub www: Option<WWWConfig>,
}

#[derive(Default, Clone)]
pub struct ServiceManager {
    files: Option<FilesService>,
    graphiql: Option<GraphiQLService>,
    graphql: Option<GraphQLService>,
    upload: Option<UploadService>,
    www: Option<WWWService>,
}

// Service Manager encapsulates (possibly async) service setup
impl ServiceManager {
    pub async fn new(config: &ServicesConfig) -> ServiceManager {
        let mut mgr = ServiceManager::default();
        let config = config.clone();

        // Load services, if config present and enabled. Async setup must be
        // done here rather than within HttpServer::new block.

        if let Some(svc_cfg) = config.files {
            if svc_cfg.enabled {
                mgr.files = Some(FilesService::new(svc_cfg));
            }
        }

        if let Some(svc_cfg) = config.graphiql {
            if svc_cfg.enabled {
                mgr.graphiql = Some(GraphiQLService::new(svc_cfg));
            }
        }

        if let Some(svc_cfg) = config.graphql {
            if svc_cfg.enabled {
                mgr.graphql = Some(GraphQLService::new(svc_cfg).await);
            }
        }

        if let Some(svc_cfg) = config.upload {
            if svc_cfg.enabled {
                mgr.upload = Some(UploadService::new(svc_cfg));
            }
        }

        if let Some(svc_cfg) = config.www {
            if svc_cfg.enabled {
                mgr.www = Some(WWWService::new(svc_cfg));
            }
        }

        println!("enabled services {:?}", &mgr.enabled_services());

        mgr
    }

    // add enabled services
    pub fn add_services<T, B>(&self, app: App<T, B>) -> App<T, B>
    where
        T: ServiceFactory<
            Config = (),
            Request = ServiceRequest,
            Response = ServiceResponse<B>,
            Error = Error,
            InitError = (),
        >,
        B: MessageBody,
    {
        let mut app = app;

        if let Some(service) = &self.files {
            app = app.service(service.factory());
        }

        if let Some(service) = &self.graphiql {
            app = app.service(service.factory());
        }

        if let Some(service) = &self.graphql {
            app = app.service(service.factory());
        }

        if let Some(service) = &self.upload {
            app = app.service(service.factory());
        }

        if let Some(service) = &self.www {
            app = app.service(service.factory());
        }

        app
    }

    pub fn enabled_services(&self) -> Vec<ServiceName> {
        let mut enabled = Vec::<ServiceName>::new();

        if self.files.is_some() {
            enabled.push(ServiceName::Files);
        }

        if self.graphiql.is_some() {
            enabled.push(ServiceName::GraphiQL);
        }
        if self.graphql.is_some() {
            enabled.push(ServiceName::GraphQL);
        }
        if self.upload.is_some() {
            enabled.push(ServiceName::Upload);
        }
        if self.www.is_some() {
            enabled.push(ServiceName::WWW);
        }

        enabled
    }
}
