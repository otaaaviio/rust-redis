use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::io::{Error, ErrorKind};
use crate::resp::parser::Parser;

pub struct RespHandler {
    stream: TcpStream,
    buffer: BytesMut,
}

impl RespHandler {
    pub fn new(stream: TcpStream) -> Self {
        RespHandler {
            stream,
            buffer: BytesMut::with_capacity(512),
        }
    }

    pub async fn get_command_with_args(&mut self) -> Result<(String, Vec<String>), Error> {
        let bytes_read = self.stream.read_buf(&mut self.buffer).await?;
        let mut command = String::new();
        let mut args = Vec::new();

        if bytes_read == 0 {
            return Err(Error::new(ErrorKind::UnexpectedEof, "No data read from stream"));
        }

        let line = String::from_utf8_lossy(&self.buffer);

        if let Some(num_args) = line.strip_prefix('*').and_then(|s| s.split("\r\n").next()?.parse::<usize>().ok()) {
            let mut parts = line.split("\r\n").skip(1);

            for _ in 0..num_args {
                if let Some(arg_len) = parts.next() {
                    if let Some(arg) = parts.next() {
                        if arg_len.starts_with('$') {
                            args.push(arg.to_string());
                        }
                    }
                }
            }

            if !args.is_empty() {
                command = args.remove(0).to_lowercase();
            }
        }

        Ok((command, args))
    }

    pub async fn response(&mut self, value: Parser) -> Result<(), Error> {
        self.stream.write_all(value.serialize().as_bytes()).await?;
        Ok(())
    }
}
