use crate::{data::models::{Transaction, User}, data_source::db_actions::add_transaction};
use actix_web::{web, Responder, HttpResponse, post, get};

pub mod data;
pub mod data_source;


#[post("/add_transaction")]
async fn record_transaction(transaction: web::Json<Transaction>) -> impl Responder {
    let transaction_data = transaction.into_inner();
    match add_transaction(transaction_data) {
        Ok(d) => HttpResponse::Ok().body(format!("Done: {}", d)),
        Err(_) => HttpResponse::BadRequest().body("Unable to accept data")
    }

}


#[get("/service_time")]
async fn show_user_waiting_time(user: web::Query<User>) -> impl Responder {
    HttpResponse::Ok().body("Hello")
}