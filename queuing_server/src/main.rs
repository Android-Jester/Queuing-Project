use actix_web::{App, HttpRequest, HttpResponse, HttpServer, post, web::Json, Responder};

fn main() {
    println!("Hello");
}


#[actix_web::main]
async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            /*Service for getting the result from the randomforest*/
            .service(best_line_service)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await

}

#[post("/")]
async fn best_line_service() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}