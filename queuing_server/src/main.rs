
mod core;
mod data_sources;
mod domain;
mod presentation;


use presentation::start_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    start_server().unwrap();
    Ok(())
}