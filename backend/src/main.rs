use actix_cors::Cors;
use actix_web::{http::KeepAlive, web, App, HttpServer};
use log;
use queuing_server::interface::teller_interface::*;
use queuing_server::interface::user_interface::*;
use queuing_server::Servers;
use std::sync::Mutex;
use std::net::Ipv4Addr;

/// Main File for runnning server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Web Server started at http://{}:3000", Ipv4Addr::UNSPECIFIED);
    HttpServer::new(|| {
        let cors = Cors::default()
            .supports_credentials()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(Mutex::new(
                queuing_server::data_source::queuing_techniques::QueueStruct::default(),
            )))
            .app_data(web::Data::new(Mutex::new(Servers::default()  )))
            .service(list_users)
            /*Teller Actions*/
            .configure(teller_config)
            /*User Actions*/
            .configure(user_config)
    })
    .keep_alive(KeepAlive::Os)
    .bind((Ipv4Addr::UNSPECIFIED, 3000))?
    .run()
    .await
}
