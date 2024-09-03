use std::net::SocketAddr;

use config::Configuration;

mod config;
mod database;
mod queue;

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() {
    // Load the configurations
    let config = Configuration::new().expect("Error loading the configurations.");
    

}