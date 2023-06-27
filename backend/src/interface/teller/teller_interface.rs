use crate::prelude::*;

use std::sync::Mutex;

#[post("/add_transaction")]
pub async fn record_transaction(transaction: web::Json<Transaction>) -> impl Responder {
    match add_transaction(transaction.into_inner()) {
        Ok(d) => HttpResponse::Ok().body(format!("Done: {}", d)),
        Err(_) => HttpResponse::BadRequest().body("Unable to accept data"),
    }
}

#[post("/status")]
pub async fn change_teller_status(
    teller_status: web::Query<bool>,
    teller_id: web::Query<String>,
) -> impl Responder {
    match set_teller_status(teller_status.into_inner(), teller_id.into_inner()) {
        Ok(data) => HttpResponse::Ok().body(data.to_string()),
        Err(d) => HttpResponse::NotAcceptable().body(d.to_string()),
    }
}

#[post("/logout")]
pub async fn logout_teller(
    teller_index: web::Query<usize>,
    tellers: web::Data<Mutex<TellersQueue>>,
) -> impl Responder {
    match tellers.lock() {
        Ok(mut teller) => {
            let _ = teller.remove_teller(teller_index.into_inner());
            HttpResponse::Ok().body("left the queue".to_string())
        }
        Err(_) => HttpResponse::Conflict().body("Teller Not Found"),
    }
}

#[post("/remove")]
pub async fn remove_user(
    user: web::Json<UserQueuePos>,
    queue_data: web::Data<Mutex<QueueStruct>>,
    server_queue: web::Data<Mutex<TellersQueue>>,
) -> impl Responder {
    // let mut mutex_server = server_queue.lock().unwrap();
    match queue_data.lock() {
        Ok(mut queue) => {
            if let Ok(mut server) = server_queue.lock() {
                if let Err(e) = queue.remove_user(user.pos, &mut server) {
                    HttpResponse::NotFound().body(e.to_string())
                } else {
                    HttpResponse::Ok().body("user leaving")
                }
            } else {
                HttpResponse::NotFound().body("Poison Error on /teller/remove")
            }
            
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string())
    }
    
}

#[get("/queue")]
pub async fn user_queues(
    user_queue_server: web::Data<Mutex<TellersQueue>>,
    // main_queue: web::Data<Mutex<QueueStruct>>,
    teller_pos: web::Query<TellerQueueStruct>,
) -> impl Responder {
    if let Ok(queue) = &mut user_queue_server.lock() {
        HttpResponse::Ok().json(queue.show_users(teller_pos.teller_position))
    } else {
        HttpResponse::NotFound().body("No Such Data")
    }
}
#[post("/login")]
pub async fn login_teller_request(
    login_data: web::Json<TellerLogin>,
    teller_queues: web::Data<Mutex<TellersQueue>>,
) -> impl Responder {
    let teller_data = login_teller(login_data.into_inner());
    match teller_data {
        Ok((teller_id, teller_loc)) => match teller_queues.lock() {
            Ok(mut teller_data) => {
                let teller_acquired = teller_data.add_teller(TellerQueueQuery {
                    server_id: teller_id,
                    server_station: teller_loc,
                });
                match teller_acquired {
                    Ok(_) => HttpResponse::Ok().json("Logged In"),
                    Err(_) => HttpResponse::NotAcceptable().body("Unable to login User"),
                }
            }
            Err(err) => HttpResponse::BadRequest().body(err.to_string()),
        },
        Err(e) => HttpResponse::NotFound().json(e),
    }
}
