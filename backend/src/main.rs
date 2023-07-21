use new_backend::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    info!(
        "Web Server started at http://{}:3000",
        Ipv4Addr::UNSPECIFIED
    );
    let broadcast_teller_lists = Broadcaster::create();
    let broadcast_countdown = BroadcasterUser::create();
    let queue_data_main: Data<Mutex<MainQueue>> = Data::new(Mutex::new(MainQueue::default()));
    let queue_data_sub: Data<Mutex<SubQueues>> = Data::new(Mutex::new(SubQueues::default()));
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
            .app_data(web::Data::from(Arc::clone(&broadcast_teller_lists)))
            .app_data(web::Data::from(Arc::clone(&broadcast_countdown)))
            .configure(teller_config) /* Teller Actions */
            .configure(user_config) /* User Actions */
    })
    .keep_alive(actix_web::http::KeepAlive::Os)
    .bind((Ipv4Addr::UNSPECIFIED, 3000))?
    .run()
    .await
}
