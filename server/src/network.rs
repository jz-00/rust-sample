use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;

use actix_http::{Request, Response};
use actix_service::{IntoServiceFactory, ServiceFactory};
use actix_web::body::MessageBody;
use actix_web::dev::AppConfig;
use actix_web::error::Error;
use actix_web::HttpServer;
use anyhow::Context;
use get_if_addrs::{get_if_addrs, IfAddr};
use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use serde::Deserialize;

use crate::config::enabled_true;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum NetworkProtocol {
    HTTP,
    HTTPS,
}

impl Display for NetworkProtocol {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::HTTPS => write!(f, "https"),
            Self::HTTP => write!(f, "http"),
        }
    }
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct SecurityConfig {
    #[serde(default = "enabled_true")]
    pub enabled: bool,
    pub cert: String,
    pub key: String,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct NetworkConfig {
    pub address: Option<String>,
    pub port: Option<u16>,
    pub security: Option<SecurityConfig>,
}

impl NetworkConfig {
    pub fn is_secure(&self) -> bool {
        if let Some(security) = &self.security {
            security.enabled
        } else {
            false
        }
    }

    pub fn protocol(&self) -> NetworkProtocol {
        if self.is_secure() {
            NetworkProtocol::HTTPS
        } else {
            NetworkProtocol::HTTP
        }
    }
}

pub struct NetworkManager {
    pub config: NetworkConfig,
}

impl NetworkManager {
    pub fn new(config: &NetworkConfig) -> NetworkManager {
        NetworkManager {
            config: config.clone(),
        }
    }

    fn get_bind_addresses(&self) -> Vec<String> {
        let mut addrs = Vec::<String>::new();
        let addr = &self.config.address;
        let port = self
            .config
            .port
            .unwrap_or(if self.config.is_secure() { 443 } else { 80 });

        match addr {
            // explicit bind address configured
            Some(addr) => addrs.push(format!("{}:{}", addr, port)),

            // no explicit bind address configured, use available IPv4 addresses
            None => {
                for if_addr in get_if_addrs().unwrap() {
                    if let IfAddr::V4(v4) = if_addr.addr {
                        addrs.push(format!("{}:{}", v4.ip.to_string(), port));
                    }
                }
            }
        }

        addrs
    }

    fn rustls_config(&self) -> anyhow::Result<ServerConfig> {
        let mut tls_config = ServerConfig::new(NoClientAuth::new());

        if let Some(security) = &self.config.security {
            // read cert/key files
            let cert_file = &mut BufReader::new(File::open(&security.cert).unwrap());
            let key_file = &mut BufReader::new(File::open(&security.key).unwrap());

            // configure cert
            let cert_chain = certs(cert_file).unwrap();
            let mut keys = rsa_private_keys(key_file).unwrap();
            tls_config
                .set_single_cert(cert_chain, keys.remove(0))
                .unwrap();
        }

        Ok(tls_config)
    }

    pub fn bind_server<F, I, S, B>(
        &self,
        server: HttpServer<F, I, S, B>,
    ) -> anyhow::Result<HttpServer<F, I, S, B>>
    where
        F: Fn() -> I + Send + Clone + 'static,
        I: IntoServiceFactory<S>,
        S: ServiceFactory<Config = AppConfig, Request = Request> + 'static,
        S::Error: Into<Error> + 'static,
        S::InitError: std::fmt::Debug,
        S::Response: Into<Response<B>> + 'static,
        B: MessageBody + 'static,
    {
        let mut server = server;
        let bind_addrs = self.get_bind_addresses();
        let tls_config = self.rustls_config()?;

        for addr in bind_addrs {
            println!("listening on {}://{}", &self.config.protocol(), addr,);

            if self.config.is_secure() {
                server = server
                    .bind_rustls(&addr, tls_config.clone())
                    .context("https address bind")?;
            } else {
                server = server.bind(&addr).context("http address bind")?;
            }
        }

        Ok(server)
    }
}
