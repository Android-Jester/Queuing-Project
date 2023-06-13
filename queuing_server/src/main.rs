use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use queuing_server::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(hello)
        .service(user_leave_queue)
        .service(record_transaction)
        .service(show_user_waiting_time)
        .service(teller_list)
        .service(change_teller_status)
        .service(logout_teller)
        .service(teller_remove_user)
    )
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}
