use crate::prelude::*;
use std::sync::Mutex;
use log::{error, info};
use serde::Deserialize;

#[post("/transaction")]
pub async fn record_transaction(transaction: web::Json<Transaction>) -> impl Responder {
    match add_transaction(transaction.into_inner()) {
        Ok(d) => HttpResponse::Ok().body(format!("Done: {}", d)),
        Err(_) => HttpResponse::BadRequest().body("Unable to accept data"),
    }
}

// #[post("/status")]
// pub async fn change_teller_status(
//     teller_status: web::Query<bool>,
//     teller_id: web::Query<String>,
// ) -> impl Responder {
//     match set_teller_status(teller_status.into_inner(), teller_id.into_inner()) {
//         Ok(data) => HttpResponse::Ok().body(data.to_string()),
//         Err(d) => HttpResponse::NotAcceptable().body(d.to_string()),
//     }
// }

#[post("/logout")]
pub async fn logout_teller(
    teller_index: web::Query<usize>,
    tellers_queue: web::Data<Mutex<SubQueues>>,
) -> impl Responder {
    match tellers_queue.lock() {
        Ok(mut teller) => {
            let _ = teller.teller_remove(teller_index.into_inner());
            HttpResponse::Ok().body("left the queue".to_string())
        }
        Err(_) => HttpResponse::Conflict().body("Teller Not Found"),
    }
}

#[post("/remove")]
pub async fn remove_user(
    user: web::Json<UserQueuePos>,
    queue_data: web::Data<Mutex<MainQueue>>,
    server_queue: web::Data<Mutex<SubQueues>>,
) -> impl Responder {
    // let mut queue = .unwrap();
    // let mut server = ;
    queue_data
        .lock()
        .unwrap()
        .remove_user(user.into_inner(), &mut server_queue.lock().unwrap());
        HttpResponse::Ok().body("Unimplemented Yet")
}

#[derive(Deserialize)]
pub struct TellerQueueStruct {
    teller_position: usize
}


#[get("/queue")]
pub async fn user_queues(
    user_queue_server: web::Data<Mutex<SubQueues>>,
    teller_loc: web::Query<TellerQueueStruct>,
) -> impl Responder {
    if let Ok(queue) = &mut user_queue_server.lock() {
        HttpResponse::Ok().json(queue.teller_show_queue(teller_loc.teller_position))
    } else {
        HttpResponse::NotFound().body("No Such Data")
    }
}
#[post("/login")]
pub async fn login_teller(
    login_data: web::Json<TellerLogin>,
    teller_queues: web::Data<Mutex<SubQueues>>,
) -> impl Responder {
    let teller_data = db_check_teller(login_data.into_inner());
    match teller_data {
        Ok((teller_id, teller_loc, service_time)) => match teller_queues.lock() {
            Ok(mut teller_data) => {
                let teller_acquired = teller_data.teller_add(TellerQueueQuery {
                    server_id: teller_id.clone(),
                    server_station: teller_loc,
                    teller_state: TellerState::Active,
                    service_time
                });
                match teller_acquired {
                    Ok(_) => {
                        info!("Logged in {}", teller_id);
                        HttpResponse::Ok().json("Logged In")
                    },
                    Err(err) => {
                        error!("ERROR: {}", err);
                        HttpResponse::NotAcceptable().body("Unable to login User")
                    },
                }
            }
            Err(err) => HttpResponse::BadRequest().body(err.to_string()),
        },
        Err(e) => HttpResponse::NotFound().json(e),
    }
}
