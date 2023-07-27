use backend::prelude::*;
use openssl::ssl::SslMethod;
use std::thread::Thread;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!(
        "Web Server started at http://{}:3000",
        std::net::Ipv4Addr::UNSPECIFIED
    );
    let broadcast_teller_lists = ServerBroadcaster::create();
    let broadcast_countdown = ClientBroadcaster::create();
    let queue_data_main: Data<Mutex<Queue>> = Data::new(Mutex::new(Queue::default()));
    let queue_data_sub: Data<Mutex<SubQueues>> = Data::new(Mutex::new(SubQueues::default()));
    // let mut builder = openssl::ssl::SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("key.pem", openssl::ssl::SslFiletype::PEM)
    //     .unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();
    // let threads_handlers: Data<Mutex<Vec<ThreadHandlers<T>>>> = Data::new(Mutex::new(Vec::<ThreadHandlers<T>>::new()))
    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .supports_credentials()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(queue_data_main.clone())
            .app_data(queue_data_sub.clone())
            .app_data(Data::from(Arc::clone(&broadcast_teller_lists)))
            .app_data(Data::from(Arc::clone(&broadcast_countdown)))
            .configure(teller_config)
            .configure(user_config)
    })
    .keep_alive(actix_web::http::KeepAlive::Os)
    .bind((Ipv4Addr::UNSPECIFIED, 3000))?
    // .bind_openssl((Ipv4Addr::UNSPECIFIED, 3000), builder)?
    .run()
    .await
}
