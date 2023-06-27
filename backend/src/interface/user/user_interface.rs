use crate::prelude::*;
use actix_web::*;
use std::sync::Mutex;

use log::{error, info};

/// Allows registered users to use the queuing service
#[post("/login")]
pub async fn user_login(login_data: web::Json<UserLogin>) -> impl Responder {
    match login_user(login_data.into_inner()) {
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
pub async fn guest_login(login_data: web::Json<Guest>) -> impl Responder {
    match login_guest(login_data.into_inner()) {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::BadRequest().body(err)
    }
}

/// Users and guests can join the main queue and assigned tellers
#[post("/join")]
pub async fn join_queue(
    user: web::Json<UserQueryData>,
    queue: web::Data<Mutex<QueueStruct>>,
    servers: web::Data<Mutex<TellersQueue>>,
) -> impl Responder {
    match find_user(user.national_id.clone()) {
        Ok(user_query) => {
            match queue.lock() {
                Ok(mut queue) => {
                    match servers.lock() {
                        Ok(mut server) => {
                            match queue.add_user(JoinedUserOutput::new(user_query, user.action.clone()), &mut server) {
                                Ok(added_user) => HttpResponse::Ok().json(added_user),
                                Err(e) => HttpResponse::NotFound().body(e.to_string()),
                            }
                        },
                        Err(err) => HttpResponse::NotFound().body(err.to_string())
                    }
                    
                }, 
                Err(err) => HttpResponse::NotFound().body(err.to_string())
            }
        }
        Err(err) => HttpResponse::NotFound().body(err) 
    }
}

/// Removes user from the queue and resets the queue
#[post("/leave")]
pub async fn leave_queue(
    user: web::Json<UserQueuePos>,
    queue: web::Data<Mutex<QueueStruct>>,
    servers: web::Data<Mutex<TellersQueue>>,
) -> impl Responder {
    match queue.lock() {
        Ok(mut queue) => {
    let user = user.into_inner();

            match servers.lock() {
                Ok(mut server) => {
                    match queue.remove_user(user.pos, &mut server) {
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
                Err(err) => HttpResponse::NotFound().body(err.to_string())
            }
            
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string())
    }
}
    

