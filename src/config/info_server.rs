use std::sync::Arc;
use crate::config::server_config::ServerConfig;
use rand::Rng;
use rand::distr::Alphanumeric;
use crate::enums::role::Role;

#[derive(Debug)]
pub struct InfoServer {
    role: Role,
    connected_slaves: u16,
    master_replid: String,
    master_repl_offset: u16,
}

impl InfoServer {
    pub fn new(config: Arc<ServerConfig>) -> Self {
        InfoServer {
            role: match config.is_replication {
                true => Role::Slave,
                false => Role::Master,
            },
            connected_slaves: 0,
            master_replid: get_random_replid(),
            master_repl_offset: 0,
        }
    }

    pub fn get_info_string(&mut self) -> String {
        format!(
            "# {}\nrole:{}\nconnected_slaves:{}\nmaster_replid:{}\nmaster_repl_offset:{}",
            "Replication",
            self.role,
            self.connected_slaves,
            self.master_replid,
            self.master_repl_offset
        )
    }
}

fn get_random_replid() -> String {
    rand::thread_rng().sample_iter(&Alphanumeric).take(40).map(char::from).collect()
}