use crate::prelude::*;
use actix_web::*;
use std::sync::Mutex;

use log::{error, info};

/// Allows registered users to use the queuing service
#[post("/login")]
pub async fn user_login(login_data: web::Json<UserLogin>) -> impl Responder {
    match db_check_user(login_data.into_inner()) {
        Ok(data) => {
            info!("User: {:?} is logged in", data);
            HttpResponse::Ok().json(data)
        }
        Err(_) => {
            error!("Invalid Login");
            HttpResponse::BadRequest().body("User Not Found")
        }
    }
}

/// Allows guests to use the service
#[post("/guest/login")]
pub async fn guest_login(guest: web::Json<GuestInsert>) -> impl Responder {
    match db_add_guest(guest.into_inner()) {
        Ok(added_guest) => {
            info!("Guest {} has loggedin", added_guest.name);
            HttpResponse::Ok().json(added_guest)
        }
        Err(err) => {
            error!("ERROR: {}", err);
            HttpResponse::BadRequest().body(err)
        }
    }
}

/// Users and guests can join the main queue and assigned tellers
#[post("/join")]
pub async fn main_queue_join(
    user_input: web::Json<UserInputData>,
    main_queue: web::Data<Mutex<MainQueue>>,
    sub_queues: web::Data<Mutex<SubQueues>>,
) -> impl Responder {
    match main_queue.lock().unwrap().add_user(
        UserQueueData::new(
            db_find_user(user_input.national_id.clone()).unwrap(),
            user_input.activity.clone(),
        ),
        &mut sub_queues.lock().unwrap(),
    ) {
        Ok(added_user) => {
            info!("Successful Join");
            HttpResponse::Ok().json(added_user)
        }
        Err(e) => {
            error!("ERROR: {}", e);
            HttpResponse::NotFound().body("Unable to add user")
        }
    }
}

/// Removes user from the queue and resets the queue
#[post("/leave")]
pub async fn main_queue_leave(
    user: web::Json<UserQueuePos>,
    main_queue: web::Data<Mutex<MainQueue>>,
    sub_queue: web::Data<Mutex<SubQueues>>,
) -> impl Responder {
    match main_queue.lock() {
        Ok(mut queue) => {
            let user = user.into_inner();

            match sub_queue.lock() {
                Ok(mut server) => match queue.remove_user(user.clone(), &mut server) {
                    Ok(_) => {
                        info!("User: {} is leaving", user.national_id);
                        HttpResponse::Ok().body(format!("user leaving: {}", user.national_id))
                    }
                    Err(e) => {
                        error!("User: {} Cannot Leave", user.national_id);
                        HttpResponse::Ok().body(e.to_string())
                    }
                },
                Err(err) => HttpResponse::NotFound().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}
