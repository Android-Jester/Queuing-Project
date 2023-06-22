use crate::data::{models::*, SERVER_COUNT};
use crate::data_source::db_actions::{add_transaction, set_teller_status};
use crate::data_source::queuing_techniques::QueueStruct;
use crate::{data_source, Servers};
use actix_web::{post, web, HttpResponse, Responder};
use std::sync::{LockResult, Mutex};
use log::info;

pub fn teller_config(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope("/teller")
            .service(record_transaction)
            .service(change_teller_status)
            .service(login_teller_request)
            .service(remove_user)
            .service(logout_teller),
    );
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
    HttpResponse::Ok().json(format!("{}", teller_status.unwrap()))
}

#[post("/logout")]
pub async fn logout_teller(teller_index: web::Query<usize>, tellers: web::Data<Mutex<TellersQueue>>) -> impl Responder {
    match tellers.lock() {
        Ok(mut teller) => {
            let left_teller = teller.remove_teller(teller_index.into_inner());
            HttpResponse::Ok().body(format!("left the queue"))
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

#[derive(Debug)]
pub struct TellersQueue {
    tellers: Vec<TellerQueueQuery>,
}

impl Default for TellersQueue {
    fn default() -> Self {
        Self {
            tellers: Vec::with_capacity(SERVER_COUNT),
        }
    }
}

impl TellersQueue {
    fn add_teller(&mut self, teller_id: TellerQueueQuery) {
        self.tellers.push(teller_id);
        info!("Added to Queue")
    }
    fn remove_teller(&mut self, index: usize) -> TellerQueueQuery {
        self.tellers.remove(index)
    }
    pub fn find_teller(&self, index: usize) -> TellerQueueQuery {
        self.tellers[index - 1].clone()
    }
}
