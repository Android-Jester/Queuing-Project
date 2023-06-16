
use std::sync::Mutex;
use actix_web::{HttpResponse, Responder, web, post, get};
use crate::data::models::{Guest, Teller, TellerLogin, Transaction, UserLogin, UserQuery};
use crate::data_source;
use crate::data_source::db_actions::{add_transaction, set_teller_status};
use crate::data_source::queuing_techniques::QueueStruct;



#[get("/server_list")]
pub async fn queue_show(teller_id: web::Query<String>) -> impl Responder {
    HttpResponse::Ok().body("Accepted")
}

#[post("/add_transaction")]
pub async fn record_transaction(transaction: web::Json<Transaction>) -> impl Responder {
    let transaction_data = transaction.into_inner();
    match add_transaction(transaction_data) {
        Ok(d) => HttpResponse::Ok().body(format!("Done: {}", d)),
        Err(_) => HttpResponse::BadRequest().body("Unable to accept data"),
    }
}

#[post("/status")]
pub async fn change_teller_status(
    teller_status: web::Query<bool>,
    teller_id: web::Query<String>,
) -> impl Responder {
    let teller_status = set_teller_status(teller_status.into_inner(), teller_id.into_inner());
    HttpResponse::Ok().body(format!("{}", teller_status.unwrap()))
}

#[post("/logout")]
pub async fn logout_teller(teller: web::Json<Teller>) -> impl Responder {
    HttpResponse::Ok().body(format!("Logged out"))
}


#[post("/remove")]
pub async fn remove_user(queue_data: web::Data<Mutex<QueueStruct<UserQuery>>>) -> impl Responder {
    let queue_mutex_data = &queue_data.into_inner();
    let mut queue = queue_mutex_data.lock().unwrap();
    match queue.remove_last_item() {
        Ok(_) => HttpResponse::Ok().body("user leaving"),
        Err(e) => HttpResponse::Ok().body(format!("{}", e))
    }
}

#[post("/login")]
pub async fn login_teller_request(login_data: web::Json<TellerLogin>) -> impl Responder {
    let teller_data = data_source::db_actions::login_teller(login_data.into_inner());
    if let Ok(_) = teller_data {
        HttpResponse::Accepted().body("Logged In")
    } else {
        HttpResponse::NotFound().body("User Not Found")
    }
}