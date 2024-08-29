use std::sync::Arc;
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::config::server_config::ServerConfig;
use crate::resp::parser::Parser::Array;

macro_rules! send_command {
    ($self:expr, $command:expr) => {{
        let command = ServerReplication::str_to_string_vec($self, $command);
        $self.stream.write_all(command.as_bytes()).await.unwrap();
        $self.stream.flush().await.unwrap();
        $self.buffer.clear();
        let bytes_read = $self.stream.read_buf(&mut $self.buffer).await.unwrap();
        if bytes_read == 0 {
            println!("No message provided from master");
        }
        let line = String::from_utf8_lossy(&$self.buffer[..]);
        let line_printable = line.replace("\r\n", "\\r\\n");
        println!("Response received from master: {}", line_printable);
        line
    }};
}

pub struct ServerReplication {
    pub(crate) stream: TcpStream,
    buffer: BytesMut,
    config: Arc<ServerConfig>,
}

impl ServerReplication {
    pub async fn new(config: Arc<ServerConfig>) -> Self {
        let addr = format!("{}:{}", config.master_host, config.master_port);
        let stream = TcpStream::connect(addr).await.unwrap();
        ServerReplication {
            stream,
            buffer: BytesMut::with_capacity(512),
            config,
        }
    }

    fn str_to_string_vec(&self, vec: Vec<&str>) -> String {
        Array(vec.into_iter().map(|s| s.to_string()).collect()).serialize()
    }

    pub async fn handshake(&mut self) {
        let res = send_command!(self, vec!["PING"]);

        if res.contains("+PONG\r\n") {
            send_command!(self, vec!["REPLCONF", "listening-port", &self.config.port.to_string()]);
            send_command!(self, vec!["REPLCONF", "capa", "psync2"]);
            send_command!(self, vec!["PSYNC", "?", "-1"]);

        }
    }
}

