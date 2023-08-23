use crate::prelude::*;

#[post("/login")]
pub async fn login_teller(
    login_data: Json<ServerLogin>,
    teller_queues: Data<Mutex<SubQueues>>,
) -> impl Responder {
    let mut sub_queue = teller_queues.lock();
    let teller_data = db_auth_teller(login_data.into_inner());
    match teller_data {
        Ok((teller_id, _, service_time)) => {
            // TODO: Change Struct to match the same result
            let teller_info = ServerQuery {
                server_id: teller_id,
                station: sub_queue.teller_count() as i32,
                active: true,
                password: "".to_string(),
                service_time,
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

#[derive(Deserialize)]
pub struct LogoutQuery {
    server_position: usize,
}

#[post("/logout")]
pub async fn logout_teller(
    server_position: Json<LogoutQuery>,
    tellers_queue: Data<Mutex<SubQueues>>,
) -> impl Responder {
    let teller_i = server_position.into_inner().server_position;
    let mut sub_queue = tellers_queue.lock();
    info!("STATION: {}", teller_i);
    match sub_queue.teller_check_state(teller_i) {
        false => match sub_queue.teller_remove(teller_i) {
            Ok(_) => {
                info!("Teller Logged Out");
                HttpResponse::Ok().body("Teller Logged Out")
            }
            Err(e) => HttpResponse::NotFound().body(e),
        },
        true => match sub_queue.teller_remove(teller_i) {
            Ok(_) => {
                info!("Teller Logged Out");
                HttpResponse::Ok().body("Teller Logged Out")
            }
            Err(e) => HttpResponse::Conflict().body(e),
        },
    }
}
