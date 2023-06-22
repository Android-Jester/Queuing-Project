use crate::data_source::queuing_techniques::QueueStruct;
use crate::interface::teller_interface::TellersQueue;
use crate::{data::models::*, data_source::db_actions::list_users_db};
use crate::{data_source, Servers};
use actix_web::{get, guard, post, web, HttpResponse, Responder};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::sync::{Mutex};

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(login_user_request)
            // .service(list_users)
            .service(login_guest_request)
            .service(user_join_queue)
            .service(user_leave_queue)
            .service(
                web::scope("/")
                    .guard(guard::Header("content-type", "text/event-stream"))
                    .guard(guard::Header("cache-control", "no-cache")), // .service(show_user_waiting_time),
            ),
    );
}

#[get("/")]
pub async fn list_users() -> impl Responder {
    let users = list_users_db().unwrap();
    info!("List all users");
    HttpResponse::Ok().json(users)
}

/*User Space*/
#[post("/login")]
pub async fn login_user_request(login_data: web::Json<UserLogin>) -> impl Responder {
    let user_data = data_source::db_actions::login_user(login_data.into_inner());
    validate(user_data)
}

#[post("/guest/login")]
pub async fn login_guest_request(login_data: web::Json<Guest>) -> impl Responder {
    let guest_data = data_source::db_actions::login_guest(login_data.into_inner());
    validate(guest_data)
}

// #[derive(Deserialize, Serialize)]
// pub struct JoinedUserOutput {
//     timer: f64,
// }

#[post("/join")]
pub async fn user_join_queue(
    user: web::Json<UserQueryData>,
    main_queue: web::Data<Mutex<QueueStruct>>,
    teller_queues: web::Data<Mutex<TellersQueue>>,
    server_queues: web::Data<Mutex<Servers>>,
) -> impl Responder {
    let mut queue = main_queue.lock().unwrap();
    let mut server = server_queues.lock().unwrap();
    let user_query = data_source::db_actions::find_user(user.national_id.clone()).unwrap();
    let teller_queue = &teller_queues.lock().unwrap();

    match queue.add_item(user_query, teller_queue, &mut server) {
        Ok(added_user) => {
            info!(
                "User: {} Joined Queue {:?}",
                added_user.national_id, queue.queue
            );
            HttpResponse::Ok().json(added_user)
        }
        Err(e) => {
            error!("User cannot Join Queue");
            HttpResponse::NotFound().body(e.to_string())
        }
    }
}
#[post("/leave")]
pub async fn user_leave_queue(
    user_data: web::Json<UserQueuePos>,
    queue_data: web::Data<Mutex<QueueStruct>>,
    servers_data: web::Data<Mutex<Servers>>,
) -> impl Responder {
    let user = user_data.into_inner();
    let mut servers = servers_data.lock().unwrap();
    let mut queue = queue_data.lock().unwrap();
    match queue.remove_item(user.queue_pos, &mut servers) {
        Ok(_) => {
            info!("User: {} is leaving", user.national_id);
            HttpResponse::Ok().body(format!("user leaving: {}", user.national_id))
        }
        Err(e) => {
            error!("User: {} Cannot Leave", user.national_id);
            HttpResponse::Ok().body(e.to_string())
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct QueryParams {
    queue_pos: usize,
    assigned_teller: usize,
}

#[derive(Deserialize, Serialize)]
struct UserInfo {
    teller_loc: usize,
    user_time: f64,
    user_teller_pos: usize,
}

pub fn show_user_waiting_time(teller_id: String, queue: &mut QueueStruct, user_pos: usize) -> f64 {
    let teller = data_source::db_actions::find_teller(teller_id);
    match teller {
        Ok(teller_data) => {
            let timer = queue.get_waiting_time(teller_data.service_time as f64, 0.0, user_pos);
            timer
        }
        Err(_) => {
            error!("User Detail unavailable");
            0.0
        }
    }
}

fn validate(data_source: Result<String, &str>) -> impl Responder {
    match data_source {
        Ok(data) => {
            info!("User: {} is logged in", data);
            HttpResponse::Ok().json(data)
        }
        Err(_) => {
            error!("Invalid Login");
            HttpResponse::BadRequest().body("User Not Found")
        }
    }
}
