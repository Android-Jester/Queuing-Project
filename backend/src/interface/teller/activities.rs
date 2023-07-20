use crate::{data_sources::queue, prelude::*};

#[post("/dismiss")]
pub async fn record_transaction(
    transaction: Json<Transaction>,
    queue_data: Data<Mutex<MainQueue>>,
    sub_queue_data: Data<Mutex<SubQueues>>,
) -> impl Responder {
    let transaction = transaction.into_inner();
    match add_transaction(transaction.clone()) {
        Ok(_) => {
            match queue_data
                .lock()
                .user_dismiss(transaction.national_id.unwrap(), &mut sub_queue_data.lock())
            {
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
    teller_index: Query<LogoutQuery>,
    tellers_queue: Data<Mutex<SubQueues>>,
) -> impl Responder {
    let teller_i = teller_index.into_inner().teller_index;
    let mut sub_queue = tellers_queue.lock();
    if sub_queue.teller_count() > 0 {
        match sub_queue.teller_check_state(teller_i) {
            TellerState::InActive | TellerState::Active => {
                match sub_queue.teller_remove(teller_i) {
                    Ok(_) => {
                        info!("Teller Logged Out");
                        HttpResponse::Ok().body("Teller Logged Out")
                    }
                    Err(e) => HttpResponse::Conflict().body(e),
                }
            }
            TellerState::PendingRelease => {
                HttpResponse::Conflict().body("Teller Already Logged Out")
            }
        }
    } else {
        HttpResponse::NotFound().body("No Teller Logged In")
    }
}

#[derive(Serialize, Deserialize)]
pub struct RemoveUserQuery {
    pub national_id: String,
}

#[post("/remove")]
pub async fn remove_user(
    // user: Json<UserQueuePos>,
    national_id: Query<RemoveUserQuery>,
    queue_data: Data<Mutex<MainQueue>>,
    server_queue: Data<Mutex<SubQueues>>,
) -> impl Responder {
    let mut queue = queue_data.lock();
    let user = queue.search_user(national_id.national_id.clone());
    match queue.user_remove(user, &mut server_queue.lock()) {
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

#[get("/")]
pub async fn tellers_listing() -> impl Responder {
    match db_list_tellers() {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

#[get("/queue")]
pub async fn user_queues(
    user_queue_server: Data<Mutex<SubQueues>>,
    teller_loc: Query<TellerQueueStruct>,
    broadcaster: Data<Broadcaster>,
) -> impl Responder {
    let queue = user_queue_server.lock();
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
    login_data: Json<TellerLogin>,
    teller_queues: Data<Mutex<SubQueues>>,
) -> impl Responder {
    let mut sub_queue = teller_queues.lock();
    let teller_data = db_auth_teller(login_data.into_inner());
    match teller_data {
        Ok((teller_id, _, service_time)) => {
            // TODO: Change Struct to match the same result
            let teller_info = TellerQuery {
                server_id: teller_id,
                server_station: sub_queue.teller_count() as i32,
                active: true,
                password: "".to_string(),
                service_time: (service_time * 60.0) as f32,
            };
            let teller_acquired = sub_queue.teller_add(teller_info.clone());
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
        Err(e) => HttpResponse::NotFound().body(e),
    }
}
