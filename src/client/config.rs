#[derive(Debug)]
pub struct ClientConfig {
    server_address: String,
}

impl ClientConfig {
    pub fn from(server_address: String) -> ClientConfig {
        ClientConfig { server_address }
    }
}
