mod parser;
mod resp;
mod storage;

use tokio::net::{TcpListener, TcpStream};
use std::io::{Error, ErrorKind};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::parser::Parser::{SimpleError, SimpleString};
use crate::resp::RespHandler;
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
                let db = Arc::clone(&storage);
                tokio::spawn(async move {
                    handle_connection(stream, db).await.unwrap()
                });
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        }
    }
}

async fn handle_connection(stream: TcpStream, storage: Arc<Mutex<Storage>>) -> Result<(), Error> {
    let mut handler = RespHandler::new(stream);

    loop {
        match handler.get_command_with_args().await {
            Ok((command, args)) => {
                println!("Command '{}' received with args: {:?}", command, args);
                match command.as_str() {
                    "ping" => {
                        handler.response(SimpleString("PONG".to_string())).await?;
                    }
                    "echo" => {
                        handler.response(SimpleString(format!("{}", args[0]))).await?;
                    }
                    "set" => {
                        let set_command_args = handler.extract_set_command_args(args).await;
                        match set_command_args {
                            Ok((key, value, exp) ) => {
                                {
                                    let mut storage = storage.lock().await;
                                    storage.set(key, value, exp)
                                }
                                handler.response(SimpleString("OK".to_string())).await?;
                            }
                            Err(e) => {
                                handler.response(SimpleError(e.to_string())).await?
                            }
                        }
                    }
                    "get" => {
                        if args.len() < 1 {
                            handler.response(SimpleError("Wrong number of arguments for 'get' command".to_string())).await?;
                        }

                        let response = {
                            {
                                let mut storage = storage.lock().await;
                                match storage.get(args[0].as_str()) {
                                    Some(item) => SimpleString(item.value.clone()),
                                    None => SimpleError("Not found".to_string()),
                                }
                            }
                        };

                        handler.response(response).await?;
                    }
                    c => {
                        handler.response(SimpleError(format!("Unknown command: {}", c))).await?;
                    }
                }
            }
            Err(e) => {
                return if e.kind() == ErrorKind::UnexpectedEof {
                    println!("Connection closed by client");
                    Ok(())
                } else {
                    Err(e)
                }
            }
        }
    }
}
