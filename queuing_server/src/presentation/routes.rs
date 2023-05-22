use actix_web::{get, HttpResponse, Responder};


#[get("/")]
pub async fn initialize_server() -> impl Responder {
    HttpResponse::Ok().body(String::from("Started"))
}

#[get("/queue")]
pub async fn queue_result() -> impl Responder {
    HttpResponse::Ok().body(String::from("Started"))
}

#[get("/complete")]
pub async fn complete_transaction() -> impl Responder {
    HttpResponse::Ok().body(String::from("Started"))
}

#[get("/incomplete")]
pub async fn incomplete_transaction() -> impl Responder {
    HttpResponse::Ok().body(String::from("Started"))
}
