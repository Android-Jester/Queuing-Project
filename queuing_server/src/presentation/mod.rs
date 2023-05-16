use actix_web::{web::get, get, HttpResponse, Responder, HttpServer, App};
use log::info;

#[get("/")]
async fn hello() -> impl Responder {
    info!("Hello There sorry for the trouble");

    HttpResponse::Accepted().body(String::from("Wrong way buddy"))
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    info!("Server Starting . . . . .");

    HttpServer::new(move || {
        info!("Server Service Beginning . . . . .");
        App::new()
            .service(hello)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
