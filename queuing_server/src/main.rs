
mod core;
mod data_sources;
mod domain;
mod presentation;
pub mod models;


use presentation::start_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    start_server().unwrap();
    Ok(())
}
