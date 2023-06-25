use crate::prelude::*;
use actix_web::*;
use std::sync::Mutex;

use log::{error, info};

/// Allows registered users to use the queuing service
#[post("/login")]
pub async fn user_login(login_data: web::Json<UserLogin>) -> impl Responder {
    let user_data = login_user(login_data.into_inner());
    validate(user_data)
}

/// Allows guests to use the service
#[post("/guest/login")]
pub async fn guest_login(login_data: web::Json<Guest>) -> impl Responder {
    let guest_data = login_guest(login_data.into_inner());
    validate(guest_data)
}

/// Users and guests can join the main queue and assigned tellers
#[post("/join")]
pub async fn join_queue(
    user: web::Json<UserQueryData>,
    queue: web::Data<Mutex<QueueStruct>>,
    servers: web::Data<Mutex<TellersQueue>>,
) -> impl Responder {
    let mut queue = queue.lock().unwrap();
    let mut server = servers.lock().unwrap();
    let user_query = find_user(user.national_id.clone()).unwrap();
    let new_struc = JoinedUserOutput {
        user_query,
        action: user.action.clone(),
    };
    match queue.add_user(new_struc, &mut server) {
        Ok(added_user) => HttpResponse::Ok().json(added_user),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

/// Removes user from the queue and resets the queue
#[post("/leave")]
pub async fn leave_queue(
    user: web::Json<UserQueuePos>,
    queue: web::Data<Mutex<QueueStruct>>,
    servers: web::Data<Mutex<TellersQueue>>,
) -> impl Responder {
    let user = user.into_inner();
    let mut servers = servers.lock().unwrap();
    let mut queue = queue.lock().unwrap();
    match queue.remove_user(user.pos, &mut servers) {
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
