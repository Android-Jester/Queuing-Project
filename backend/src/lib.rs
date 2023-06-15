use std::sync::Mutex;
use actix_web::{HttpResponse, Responder, web, post, get};
use crate::data::models::{Guest, Teller, TellerLogin, Transaction, UserLogin, UserQuery};
use crate::data_source::db_actions::{add_transaction, set_teller_status};
use crate::data_source::queuing_techniques::QueueStruct;

pub mod data;
pub mod data_source;
#[cfg(test)]
pub mod tests;








/*User Space*/
#[post("/user/login")]
async fn login_user_request(login_data: web::Json<UserLogin>) -> impl Responder {
     let user_data = data_source::db_actions::login_user(login_data.into_inner());
    if let Ok(_) = user_data {
        HttpResponse::Accepted().body("Logged In")
    } else {
        HttpResponse::NotFound().body("User Not Found")
    }
}




#[post("/guest/login")]
async fn login_guest_request(login_data: web::Json<Guest>) -> impl Responder {
     let guest_data = data_source::db_actions::login_guest(login_data.into_inner());
    if let Ok(_) = guest_data {
        HttpResponse::Accepted().body("Logged In")
    } else {
        HttpResponse::NotFound().body("User Not Found")
    }
}




// #[post("/user/join")]
pub async fn user_join_queue(user: web::Json<UserQuery>, queue_data: web::Data<Mutex<QueueStruct<UserQuery>>>) -> impl std::future::Future<Output = impl Responder> {
    let queue_data = &queue_data.into_inner();
    let mut queue = queue_data.lock().unwrap();
    match queue.add_item(user.into_inner()) {
        Ok(_) => HttpResponse::Ok().body("user joining"),
        Err(e) => HttpResponse::Ok().body(format!("{}", e))
    }

}

#[post("/user/leave")]
pub async fn user_leave_queue(user: web::Json<UserQuery>, queue_data: web::Data<Mutex<QueueStruct<UserQuery>>>) -> impl Responder {
    let queue_mutex_data = &queue_data.into_inner();
    let mut queue = queue_mutex_data.lock().unwrap();
    match queue.remove_last_item() {
        Ok(_) => HttpResponse::Ok().body("user leaving"),
        Err(e) => HttpResponse::Ok().body(format!("{}", e))
    }
}

#[get("/user/time")]
pub async fn show_user_waiting_time(user: web::Json<UserQuery>) -> impl Responder {
    // let user = user.into_inner();
    // let timer = get_waiting_time(user.account_number);
    // for i in (0..timer).rev() {
    //     thread::sleep(Duration::from_secs(1));
    // }
    HttpResponse::Ok().body("Hello")
}

/*Tellers Space*/

#[post("/teller/add_transaction")]
pub async fn record_transaction(transaction: web::Json<Transaction>) -> impl Responder {
    let transaction_data = transaction.into_inner();
    match add_transaction(transaction_data) {
        Ok(d) => HttpResponse::Ok().body(format!("Done: {}", d)),
        Err(_) => HttpResponse::BadRequest().body("Unable to accept data"),
    }
}

#[post("/teller/login")]
async fn login_teller_request(login_data: web::Json<TellerLogin>) -> impl Responder {
    let teller_data = data_source::db_actions::login_teller(login_data.into_inner());
    if let Ok(_) = teller_data {
        HttpResponse::Accepted().body("Logged In")
    } else {
        HttpResponse::NotFound().body("User Not Found")
    }
}




#[get("/teller/server_list")]
pub async fn queue_show(teller_id: web::Query<String>) -> impl Responder {
    HttpResponse::Ok().body("Accepted")
}

#[post("/teller/status")]
pub async fn change_teller_status(
    teller_status: web::Query<bool>,
    teller_id: web::Query<String>,
) -> impl Responder {
    let teller_status = set_teller_status(teller_status.into_inner(), teller_id.into_inner());
    HttpResponse::Ok().body(format!("{}", teller_status.unwrap()))
}

#[post("/teller/logout")]
pub async fn logout_teller(teller: web::Json<Teller>) -> impl Responder {
    HttpResponse::Ok().body(format!("Logged out"))
}


#[post("/teller/remove")]
pub async fn remove_user(queue_data: web::Data<Mutex<QueueStruct<UserQuery>>>) -> impl Responder {
    let queue_mutex_data = &queue_data.into_inner();
    let mut queue = queue_mutex_data.lock().unwrap();
    match queue.remove_last_item() {
        Ok(_) => HttpResponse::Ok().body("user leaving"),
        Err(e) => HttpResponse::Ok().body(format!("{}", e))
    }
}
