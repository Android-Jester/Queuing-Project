mod core;
mod features;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
#[get("/")]
async fn hello() -> impl Responder {
    // HttpResponse::Ok().body(String::from("Hello World"))
    HttpResponse::Accepted().body(String::from("Wrong way buddy"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(hello))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
