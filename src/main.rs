extern crate core;

mod storage;
mod commands;
mod resp;
mod errors;
mod servers;
mod config;
mod enums;
#[macro_use]
mod macros;
mod constants;

use std::env::args;
use tokio::net::TcpListener;
use std::sync::Arc;
use std::time::{Duration};
use tokio::sync::Mutex;
use crate::commands::handler::handle_connection;
use crate::config::info_server::InfoServer;
use crate::config::server_config::{get_server_config};
use crate::servers::replication::ServerReplication;
use crate::storage::Storage;

#[tokio::main]
async fn main() {
    let config = Arc::new(get_server_config(args()));
    let listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).await.unwrap();
    let storage = Arc::new(Mutex::new(Storage::new()));
    let info_server = Arc::new(Mutex::new(InfoServer::new(Arc::clone(&config))));

    if config.is_replication {
        let config_clone = Arc::clone(&config);
        let mut replication_server = ServerReplication::new(config_clone).await;
        replication_server.handshake().await;
    } else {
        load_rdb_file!(storage);
    }

    init_snapshotting!(storage, tokio);

    loop {
        let res = listener.accept().await;

        match res {
            Ok((stream, addr)) => {
                println!("Accepted connection from {addr}");
                let storage_clone = Arc::clone(&storage);
                let info_server_clone = Arc::clone(&info_server);
                tokio::spawn(async move {
                    handle_connection(stream, storage_clone, info_server_clone).await.unwrap()
                });
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        }
    }
}

