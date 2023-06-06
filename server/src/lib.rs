#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

mod config;
mod network;
mod service;
mod util;

use actix_cors::Cors;
use actix_web::{http::header, middleware, App, HttpServer};

use config::ServerConfig;
use network::NetworkManager;
use service::manager::ServiceManager;

pub async fn run() -> anyhow::Result<()> {
    let server_config = ServerConfig::load()?;
    let network_manager = NetworkManager::new(&server_config.network);
    let service_manager = ServiceManager::new(&server_config.services).await;

    let server = {
        let service_manager = service_manager.clone();
        HttpServer::new(move || {
            let cors = Cors::new()
                .allowed_methods(vec!["POST", "GET"])
                .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
                .max_age(3600)
                .finish();

            let app = App::new()
                .wrap(middleware::Compress::default())
                .wrap(middleware::Logger::default())
                .wrap(cors);

            service_manager.add_services(app)
        })
    };

    network_manager
        .bind_server(server)?
        .run()
        .await
        .map_err(anyhow::Error::new)
}
