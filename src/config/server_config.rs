#[derive(Debug)]
pub struct ServerConfig {
    pub(crate) port: u16,
    pub(crate) host: String,
    pub(crate) master_port: u16,
    pub(crate) master_host: String,
    pub(crate) is_replication: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig{
            port: 6379,
            host: "127.0.0.1".to_string(),
            master_port: 6379,
            master_host: "".to_string(),
            is_replication: false,
        }
    }
}

pub fn get_server_config(args: std::env::Args) -> ServerConfig {
    let mut config = ServerConfig::default();
    let mut args_iter = args.skip(1);

    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "--port" | "-p" => {
                if let Some(port_str) = args_iter.next() {
                    if let Ok(port) = port_str.parse::<u16>() {
                        config.port = port;
                    }
                }
            }
            "--replicaof"  | "-r" => {
                config.is_replication = true;
                if let Some(replicaof) = args_iter.next() {
                    let parts: Vec<&str> = replicaof.split(' ').collect();
                    if parts.len() == 2 {
                        config.master_host = parts[0].to_string();
                        if let Ok(port) = parts[1].parse::<u16>() {
                            config.master_port = port;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    config
}