extern crate core;

mod storage;
mod commands;
mod resp;
mod errors;
mod servers;
mod config;
mod enums;

use std::env::args;
use tokio::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::commands::handler::handle_connection;
use crate::config::server_config::{get_server_config};
use crate::storage::Storage;

#[tokio::main]
async fn main() {
    let config = Arc::new(get_server_config(args()));
    let storage = Arc::new(Mutex::new(Storage::new()));
    let listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).await.unwrap();

    loop {
        let res = listener.accept().await;

        match res {
            Ok((stream, addr)) => {
                println!("Accepted connection from {addr}");
                let storage_clone = Arc::clone(&storage);
                let config_clone = Arc::clone(&config);
                tokio::spawn(async move {
                    handle_connection(stream, storage_clone, config_clone).await.unwrap()
                });
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        }
    }
}

