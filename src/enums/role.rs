use std::fmt::Display;

#[derive(Debug)]
pub enum Role {
    Master,
    Slave,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Role::Master => "master",
            Role::Slave => "slave",
        })
    }
}