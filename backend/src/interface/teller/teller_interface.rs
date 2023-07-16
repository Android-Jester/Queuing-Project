use crate::{interface::sse::broadcaster::Broadcaster, prelude::*};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::{sync::Mutex, time::Duration};

#[post("/dismiss")]
pub async fn record_transaction(
    transaction: web::Json<Transaction>,
    queue_data: web::Data<Mutex<MainQueue>>,
    sub_queue_data: web::Data<Mutex<SubQueues>>,
) -> impl Responder {
    let transaction = transaction.into_inner();
    match add_transaction(transaction.clone()) {
        Ok(d) => {
            match queue_data.lock().unwrap().dismiss_user(
                transaction.national_id.unwrap(),
                &mut sub_queue_data.lock().unwrap(),
            ) {
                Ok(_) => {
                    info!("Transaction Recorded");
                    HttpResponse::Ok().body("Transaction Recorded")
                }
                Err(e) => {
                    error!("ERROR: {}", e);
                    HttpResponse::NotFound().body(e)
                }
            }
        }
        Err(_) => HttpResponse::BadRequest().body("Unable to accept data"),
    }
}

#[derive(Deserialize)]
pub struct LogoutQuery {
    pub teller_index: usize,
}

#[post("/logout")]
pub async fn logout_teller(
    teller_index: web::Query<LogoutQuery>,
    tellers_queue: web::Data<Mutex<SubQueues>>,
) -> impl Responder {
    let teller_i = teller_index.into_inner().teller_index;
    match tellers_queue.lock() {
        Ok(mut teller) => {
            if teller.teller_count() > 0 {
                match teller.teller_check_state(teller_i) {
                    TellerState::InActive | TellerState::Active => {
                        match teller.teller_remove(teller_i) {
                            Ok(_) => {
                                info!("Teller Logged Out");
                                HttpResponse::Ok().body("Teller Logged Out")
                            }
                            Err(e) => HttpResponse::Conflict().body(e),
                        }
                    }
                    TellerState::PendingRelease => {
                        return HttpResponse::Conflict().body("Teller Already Logged Out")
                    }
                }
            } else {
                HttpResponse::NotFound().body("No Teller Logged In")
            }
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
    match queue_data
        .lock()
        .unwrap()
        .remove_user(user.into_inner(), &mut server_queue.lock().unwrap())
    {
        Ok(_) => {
            info!("User Removed");
            HttpResponse::Ok().body("User Removed")
        }
        Err(e) => {
            error!("ERROR: {}", e);
            HttpResponse::NotFound().body(e)
        }
    }
}

#[derive(Deserialize)]
pub struct TellerQueueStruct {
    teller_position: usize,
}

#[get("/tt")]
pub async fn tellers_listing() -> impl Responder {
    match db_list_tellers() {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

#[get("/queue")]
pub async fn user_queues(
    user_queue_server: web::Data<Mutex<SubQueues>>,
    teller_loc: web::Query<TellerQueueStruct>,
    broadcaster: web::Data<Broadcaster>,
) -> impl Responder {
    let queue = user_queue_server.lock().unwrap();
    let json_data = queue.teller_show_queue(teller_loc.teller_position);
    info!("Data: {:?}", json_data);
    broadcaster.new_client(&json_data).await

    // if let Ok(queue) = &mut user_queue_server.lock() {
    //     // HttpResponse::Ok()
    //     //     .content_type("value")
    //     //     .json()
    // }
    // else {
    //     HttpResponse::NotFound().body("No Such Data")
    // }
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
                let teller_info = TellerQueueQuery {
                    server_id: teller_id,
                    server_station: teller_loc,
                    teller_state: TellerState::Active,
                    service_time: Duration::from_secs_f64(service_time * 60.0),
                };
                let teller_acquired = teller_data.teller_add(teller_info.clone());
                match teller_acquired {
                    Ok(_) => {
                        info!("Logged in {:?}", teller_info);
                        HttpResponse::Ok().json(teller_info)
                    }
                    Err(err) => {
                        error!("ERROR: {}", err);
                        HttpResponse::NotAcceptable().body("Unable to login User")
                    }
                }
            }
            Err(err) => HttpResponse::BadRequest().body(err.to_string()),
        },
        Err(e) => HttpResponse::NotFound().json(e),
    }
}
