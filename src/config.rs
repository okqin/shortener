use crate::Error;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    pub database: Database,
    pub server_addr: SocketAddr,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub url: String,
    pub pool_size: u32,
}

const DEFAULT_DATABASE_POOL_SIZE: u32 = 100;
const DEFAULT_DATABASE_URL: &str = "postgres://localhost:5432/shortener";
const DEFAULT_SERVER_HOST: &str = "0.0.0.0";

impl Config {
    pub fn load() -> Result<Self, Error> {
        let database_url = std::env::var("DATABASE_URL")
            .ok()
            .unwrap_or(DEFAULT_DATABASE_URL.to_string());
        let database_pool_size = std::env::var("DATABASE_POOL_SIZE")
            .ok()
            .and_then(|pool_size_str| pool_size_str.parse().ok())
            .unwrap_or(DEFAULT_DATABASE_POOL_SIZE);

        let database = Database {
            url: database_url,
            pool_size: database_pool_size,
        };

        let host = std::env::var("SERVER_HOST")
            .ok()
            .unwrap_or(DEFAULT_SERVER_HOST.to_string());
        let port = std::env::var("SERVER_PORT")
            .ok()
            .and_then(|port_str| port_str.parse().ok())
            .unwrap_or(8080);
        let server_addr = format!("{}:{}", host, port)
            .parse()
            .map_err(|err| Error::Internal(format!("invalid server address: {}", err)))?;

        let config = Config {
            database,
            server_addr,
        };

        Ok(config)
    }
}
