use crate::data::models::*;
use crate::data_source::db_actions::{add_transaction, set_teller_status};
use crate::data_source::queuing_techniques::QueueStruct;
use crate::{data_source, Servers};
use actix_web::{post, web, HttpResponse, Responder};
use std::sync::Mutex;

pub fn teller_config(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope("/teller")
            .service(record_transaction)
            .service(change_teller_status)
            .service(login_teller_request)
            .service(remove_user)
            .service(logout_teller),
        // .service(queue_show),
    );
}

// #[get("/server_list")]
// pub async fn queue_show(
//     teller_id: web::Json<String>,
//     queue_data: web::Data<Mutex<QueueStruct>>,
// ) -> impl Responder {
//     // let queue_length = &queue_data.lock().unwrap().queue_len();

//     // let teller: Teller;
//     // let current_queue = queue_data.lock().unwrap();

//     // for index in 0..*queue_length {
//     //     let current_pos = index % SERVER_COUNT;
//     //     if current_queue.get_user(index) {
//     //         teller_queue
//     //     }
//     // }

//     HttpResponse::Ok().body(format!("{}", teller_id))
// }

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
    HttpResponse::Ok().body(format!("Logged out: {}", teller))
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
pub async fn login_teller_request(login_data: web::Json<TellerLogin>) -> impl Responder {
    let teller_data = data_source::db_actions::login_teller(login_data.into_inner());
    if teller_data.is_ok() {
        HttpResponse::Accepted().body("Logged In")
    } else {
        HttpResponse::NotFound().body("User Not Found")
    }
}
