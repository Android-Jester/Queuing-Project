
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use actix_web::{HttpResponse, Responder, web, post, get};
use crate::data::models::{Guest, Teller, TellerLogin, Transaction, UserLogin, UserQuery};
use crate::data_source;
use crate::data_source::db_actions::{add_transaction, set_teller_status};
use crate::data_source::queuing_techniques::QueueStruct;


/*User Space*/
#[post("/login")]
pub async fn login_user_request(login_data: web::Json<UserLogin>) -> impl Responder {
    let user_data = data_source::db_actions::login_user(login_data.into_inner());
    if let Ok(_) = user_data {
        HttpResponse::Accepted().body("Logged In")
    } else {
        HttpResponse::NotFound().body("User Not Found")
    }
}




#[post("/guest/login")]
pub async fn login_guest_request(login_data: web::Json<Guest>) -> impl Responder {
    let guest_data = data_source::db_actions::login_guest(login_data.into_inner());
    if let Ok(_) = guest_data {
        HttpResponse::Accepted().body("Logged In")
    } else {
        HttpResponse::NotFound().body("User Not Found")
    }
}




#[post("/join")]
pub async fn user_join_queue(user: web::Json<UserQuery>, queue_data: web::Data<Mutex<QueueStruct<UserQuery>>>) -> impl Responder {
    let queue_data = &queue_data.into_inner();
    let mut queue = queue_data.lock().unwrap();
    match queue.add_item(user.into_inner()) {
        Ok(_) => HttpResponse::Ok().body("user joining"),
        Err(e) => HttpResponse::Ok().body(format!("{}", e))
    }

}

#[post("/leave")]
pub async fn user_leave_queue(user: web::Json<UserQuery>, queue_data: web::Data<Mutex<QueueStruct<UserQuery>>>) -> impl Responder {
    let queue_mutex_data = &queue_data.into_inner();
    let mut queue = queue_mutex_data.lock().unwrap();
    match queue.remove_last_item() {
        Ok(_) => HttpResponse::Ok().body("user leaving"),
        Err(e) => HttpResponse::Ok().body(format!("{}", e))
    }
}

#[get("/time")]
pub async fn show_user_waiting_time(user_query: web::Query<UserQuery>, queue_data: web::Data<Mutex<QueueStruct<UserQuery>>>) -> impl Responder {
    // let user = user_query.into_inner();
    // let timer = queue_data.lock().unwrap().get_waiting_time(user.account_number);
    // for i in (0..timer).rev() {
    //     thread::sleep(Duration::from_secs(1));
    // }
    HttpResponse::Ok().body("Time")
}
