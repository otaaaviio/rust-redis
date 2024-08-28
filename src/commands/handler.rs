use std::io::{Error, ErrorKind};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use crate::storage::Storage;
use std::{format, println};
use crate::errors::app_errors::AppError;
use crate::resp::handler::RespHandler;
use crate::resp::parser::extract_set_command_args;
use crate::resp::parser::Parser::{Integer, NullBulkString, SimpleError, SimpleString};

pub async fn handle_connection(stream: TcpStream, storage: Arc<Mutex<Storage>>) -> Result<(), Error> {
    let mut handler = RespHandler::new(stream);

    loop {
        match handler.get_command_with_args().await {
            Ok((command, args)) => {
                println!("Command '{}' received with args: {:?}", command, args);
                match command.as_str() {
                    "ping" => handler.response(SimpleString("PONG".to_string())).await?,
                    "echo" => {
                        if !args.is_empty() {
                            handler.response(SimpleString(args[0].to_string())).await?
                        } else {
                            handler.response(SimpleError(AppError::WrongNumberOfArgumentsError.to_string())).await?;
                        }
                    }
                    "set" => {
                        let set_command_args = extract_set_command_args(args).await;
                        match set_command_args {
                            Ok((key, value, exp)) => {
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
                            handler.response(SimpleError(AppError::WrongNumberOfArgumentsError.to_string())).await?;
                            return Ok(())
                        }

                        let response = {
                            {
                                let mut storage = storage.lock().await;
                                match storage.get(args[0].as_str()) {
                                    Some(item) => SimpleString(item.value.clone()),
                                    None => NullBulkString,
                                }
                            }
                        };

                        handler.response(response).await?;
                    }
                    "del" => {
                        if args.len() < 1 {
                            handler.response(SimpleError(AppError::WrongNumberOfArgumentsError.to_string())).await?;
                            return Ok(())
                        }

                        let count_deleted_keys = {
                            let mut storage = storage.lock().await;
                            storage.del(args.iter().map(|s| s.as_str()).collect())
                        };

                        handler.response(Integer(None, count_deleted_keys)).await?
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
