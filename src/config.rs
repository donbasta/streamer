use crate::{client::config::ClientConfig, server::config::ServerConfig};

pub enum Config {
    Server(ServerConfig),
    Client(ClientConfig),
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let user_type = match args.next() {
            Some(arg) => arg,
            None => return Err("Not enough arguments"),
        };

        let config: Config;

        match user_type.as_str() {
            "client" => {
                let server_address = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Not enough arguments"),
                };
                config = Config::Client(ClientConfig::from(server_address));
            }
            "server" => {
                let port = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Not enough arguments"),
                };
                let source_path = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Not enough arguments"),
                };
                config = Config::Server(ServerConfig::from(port, source_path));
            }
            _ => return Err("Argument not valid"),
        }

        Ok(config)
    }
}
