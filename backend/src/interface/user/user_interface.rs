// use parking_lot::Mutex;
use std::sync::Mutex;

use crate::{interface::sse::broadcaster::Broadcaster, prelude::*};
use actix_web::*;

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
    broadcast_agent: web::Data<Broadcaster>,
) -> impl Responder {
    // let sub_queue = sub_queues.lock().unwrap();
    let user_name = db_find_user(user_input.national_id.clone()).unwrap().name;
    let mut sub_queue = sub_queues.lock().unwrap();
    // let mut tellers_service_times:[f64; SERVER_COUNT] = [0.0; SERVER_COUNT];
    // let _ = sub_queue.tellers.iter().map(|data| {
    //     tellers_service_times[data.teller.server_station as usize] = data.teller.service_time.as_secs_f64() / 60.0;
    // });
    // let prediction = prediction(tellers_service_times) as usize;

    match main_queue.lock().unwrap().add_user(
        UserQueuePos::new(user_input.into_inner(), user_name, 1),
        &mut sub_queue,
    ) {
        Ok(added_user) => {
            info!("Successful Join");
            broadcast_agent
                .broadcast(&sub_queue, added_user.service_location)
                .await;
            HttpResponse::Ok().json(added_user)
        }
        Err(e) => {
            error!("ERROR: {}", e);
            HttpResponse::NotFound().body(e)
        }
    }
}

/// Removes user from the queue and resets the queue
#[post("/leave")]
pub async fn main_queue_leave(
    user: web::Json<UserQueuePos>,
    main_queue: web::Data<Mutex<MainQueue>>,
    sub_queue: web::Data<Mutex<SubQueues>>,
    broadcast_agent: web::Data<Broadcaster>,
) -> impl Responder {
    info!("Attempted leaving: {:?}", user);
    // let mut queue = main_queue.lock();
    // let mut sub_queue = sub_queue.lock();
    // let user = user.into_inner();
    // // let removed_user = ;
    // match queue.remove_user(user, &mut sub_queue) {
    //     Ok(user) => {
    //         broadcast_agent
    //             .broadcast(&sub_queue, user.service_location)
    //             .await;
    //         HttpResponse::Ok().body(format!("Successfully removed: {}", user.national_id))
    //     }
    //     Err(err) => {
    //         error!("ERROR: {}", err);
    //         HttpResponse::NotFound().body(err)
    //     }
    // }

    match main_queue.lock() {
        Ok(mut queue) => {
            let user = user.into_inner();

            match sub_queue.lock() {
                Ok(mut server) => {
                    let removed_user = queue.remove_user(user.clone(), &mut server);
                    match removed_user {
                        Ok(removed_user) => {
                            broadcast_agent
                                .broadcast(&server, removed_user.service_location)
                                .await;
                            info!("Successful Leave");
                            HttpResponse::Ok().body(format!("Removed: {:?}", removed_user))
                        }
                        Err(err) => {
                            error!("ERROR: {}", err);
                            HttpResponse::NotFound().body(err)
                        }
                    }
                }

                Err(err) => HttpResponse::NotFound().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}
