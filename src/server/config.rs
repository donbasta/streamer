#[derive(Debug)]
pub struct ServerConfig {
    port: String,
    source_path: String,
}

impl ServerConfig {
    pub fn from(port: String, source_path: String) -> ServerConfig {
        ServerConfig { source_path, port }
    }
}
