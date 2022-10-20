use std::{env, error::Error, process};

use lib::{
    client::{config::ClientConfig, item::Client},
    config::Config,
    server::{config::ServerConfig, item::Server},
};

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config {
        Config::Server(server_config) => run_server(server_config),
        Config::Client(client_config) => run_client(client_config),
    }
}

fn run_server(config: ServerConfig) -> Result<(), Box<dyn Error>> {
    println!("starting server with config {:#?}", config);
    let server = Server::from(config);
    Ok(())
}

fn run_client(config: ClientConfig) -> Result<(), Box<dyn Error>> {
    println!("starting client with config {:#?}", config);
    let client = Client::from(config);
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
