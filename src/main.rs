use std::{env, error::Error, process};

#[derive(Debug)]
struct ClientConfig {
    server_address: String,
}
#[derive(Debug)]
struct ServerConfig {
    port: String,
    fpath: String,
}

enum Config {
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
                config = Config::Client(ClientConfig { server_address });
            }
            "server" => {
                let port = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Not enough arguments"),
                };
                let fpath = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Not enough arguments"),
                };
                config = Config::Server(ServerConfig { port, fpath });
            }
            _ => return Err("Argument not valid"),
        }

        Ok(config)
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config {
        Config::Server(server_config) => run_server(server_config),
        Config::Client(client_config) => run_client(client_config),
    }
}

fn run_server(config: ServerConfig) -> Result<(), Box<dyn Error>> {
    println!("starting server with config {:#?}", config);
    Ok(())
}

fn run_client(config: ClientConfig) -> Result<(), Box<dyn Error>> {
    println!("starting client with config {:#?}", config);
    Ok(())
}

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(e) = run(config) {
        eprintln!("Application error with error message {e}");
        process::exit(1);
    }
}
