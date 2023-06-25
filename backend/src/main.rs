use actix_cors::Cors;
use actix_web::{http::KeepAlive, *};
use queuing_server::prelude::*;
use std::net::Ipv4Addr;
use std::sync::Mutex;

/// Main File for runnning server

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!(
        "Web Server started at http://{}:3000",
        Ipv4Addr::UNSPECIFIED
    );

    let queue_struct: web::Data<Mutex<QueueStruct>> =
        web::Data::new(Mutex::new(QueueStruct::default()));
    let user_servers: web::Data<Mutex<TellersQueue>> =
        web::Data::new(Mutex::new(TellersQueue::default()));
    HttpServer::new(move || {
        let cors: Cors = Cors::default()
            .supports_credentials()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(queue_struct.clone())
            .app_data(user_servers.clone())
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
