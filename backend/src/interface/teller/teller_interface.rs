use std::sync::Mutex;
use crate::*;
use crate::{data::models::*, data_source::{db_actions::*, queuing_techniques::*}};
use actix_web::{web, HttpResponse, Responder, post, get};
use log::info;
// use crate::data_source::queuing_techniques::QueueStruct;
// use crate::{data_source, Servers};
// use actix_web::{post, web, HttpResponse, Responder, get};
// use serde::{Deserialize, Serialize};



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
    HttpResponse::Ok().json(format!("{}", teller_status.unwrap()))
}

#[post("/logout")]
pub async fn logout_teller(teller_index: web::Query<usize>, tellers: web::Data<Mutex<TellersQueue>>) -> impl Responder {
    match tellers.lock() {
        Ok(mut teller) => {
            teller.remove_teller(teller_index.into_inner());
            HttpResponse::Ok().body("left the queue".to_string())
        }
        Err(_) => {
            HttpResponse::Conflict().body("Teller Not Found")
        }
    }
}

#[post("/remove")]
pub async fn remove_user(
    queue_data: web::Data<Mutex<QueueStruct>>,
    server_queue: web::Data<Mutex<Servers>>,
) -> impl Responder {
    let queue_mutex_data = &queue_data.into_inner();
    let server = &server_queue.into_inner();
    let mut mutex_server = server.lock().unwrap();
    let mut queue = queue_mutex_data.lock().unwrap();
    let queue_length = &queue.queue.len();
    match queue.remove_item(*queue_length, &mut mutex_server) {
        Ok(_) => HttpResponse::Ok().body("user leaving"),
        Err(e) => HttpResponse::Ok().body(e.to_string()),
    }
}




#[get("/queue")]
pub async fn user_queues(user_queue_server: web::Data<Mutex<Servers>>, /*main_queue: web::Data<Mutex<QueueStruct>>, */ teller_pos: web::Query<TellerQueueStruct>) -> impl Responder {
    if let Ok(queue) = user_queue_server.lock() {
    HttpResponse::Ok().json(queue.show_users(teller_pos.teller_position))
    }
    else {
        HttpResponse::NotFound().body("No Such Data")
    }
}




#[post("/login")]
pub async fn login_teller_request(
    login_data: web::Json<TellerLogin>,
    teller_queues: web::Data<Mutex<TellersQueue>>,
) -> impl Responder {
    let teller_data = data_source::db_actions::login_teller(login_data.into_inner());

    match teller_data {
        Ok((teller_id, teller_loc)) => {
            if let Ok(mut teller_data) = teller_queues.lock() {
                let teller_queue_query = TellerQueueQuery {
                    server_id: teller_id,
                    server_station: teller_loc,
                };
                teller_data.add_teller(teller_queue_query)
            }
            info!("Queue Data: {:?}", teller_queues.lock());
            HttpResponse::Ok().json("Logged In")
        }
        Err(e) => HttpResponse::NotFound().json(e),
    }
}

