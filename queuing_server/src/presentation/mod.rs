use actix_web::{web::get, get, HttpResponse, Responder, HttpServer, App};
use log::info;
mod routes;
use routes::*;
#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    info!("Server Starting . . . . .");

    HttpServer::new(move || {
        info!("Server Service Beginning . . . . .");
        App::new()
            .service(initialize_server)
            .service(queue_result)
            .service(complete_transaction)
            .service(incomplete_transaction)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
