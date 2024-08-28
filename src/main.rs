mod storage;
mod commands;
mod resp;
mod errors;

use tokio::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::commands::handler::handle_connection;
use crate::storage::Storage;

#[tokio::main]
async fn main() {
    let storage = Arc::new(Mutex::new(Storage::new()));

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let res = listener.accept().await;

        match res {
            Ok((stream, addr)) => {
                println!("Accepted connection from {addr}");
                let storage_clone = Arc::clone(&storage);
                tokio::spawn(async move {
                    handle_connection(stream, storage_clone).await.unwrap()
                });
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        }
    }
}

