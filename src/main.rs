mod domain;
mod ports;
mod adapters;

use tokio;
use hyper::Server;
use hyper::service::{make_service_fn, service_fn};
use ports::config_port::EnvConfigPort;
use std::sync::Arc;
use crate::adapters::config_adapter::FileConfigAdapter;
use crate::adapters::env_config_adapter::EnvConfigAdapter;
use crate::ports::config_port::ConfigPort;
use crate::adapters::http_adapter::handle_request;
use env_logger;
use log::info;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)  // Imposta il livello su info
        .init();
    info!("Starting proxy...");

    let config_adapter = FileConfigAdapter::new("Config.yaml".to_string());
    let env_config_adapter = EnvConfigAdapter::new();
    let mut config = config_adapter.load_config();
    if let Some(env_address) = env_config_adapter.load_proxy_address() {
        config.proxy.address = env_address;
    }
    let rules = Arc::new(config.proxy.rules);
    let addr = config.proxy.address.parse().unwrap();


    let make_svc = make_service_fn(move |_conn| {
        let rules = Arc::clone(&rules);
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let rules = Arc::clone(&rules);
                handle_request(req, rules)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
