use crate::data_source::queuing_techniques::QueueStruct;
use crate::{data::models::*, data_source::db_actions::list_users_db};
use crate::{data_source, Servers};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Mutex;

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(login_user_request)
            .service(list_users)
            .service(login_guest_request)
            .service(user_join_queue)
            .service(user_leave_queue)
            .service(show_user_waiting_time),
    );
}

#[get("/list")]
async fn list_users() -> impl Responder {
    let users = list_users_db().unwrap();
    web::Json(users)
}

/*User Space*/
#[post("/login")]
pub async fn login_user_request(login_data: web::Json<UserLogin>) -> impl Responder {
    let user_data = data_source::db_actions::login_user(login_data.into_inner());
    if let Ok(_) = user_data {
        HttpResponse::Accepted().body("Logged In")
    } else {
        HttpResponse::NotFound().body("User Not Found")
    }
}

#[post("/guest/login")]
pub async fn login_guest_request(login_data: web::Json<Guest>) -> impl Responder {
    let guest_data = data_source::db_actions::login_guest(login_data.into_inner());
    if let Ok(_) = guest_data {
        HttpResponse::Accepted().body("Logged In")
    } else {
        HttpResponse::NotFound().body("User Not Found")
    }
}

#[post("/join")]
pub async fn user_join_queue(
    user: web::Json<UserQueuePos>,
    main_queue: web::Data<Mutex<QueueStruct>>,
    server_queues: web::Data<Mutex<Servers>>,
) -> impl Responder {
    let queue_data = &main_queue.into_inner();
    let mut queue = queue_data.lock().unwrap();
    let server = &server_queues.into_inner();
    let mut mutex_server = server.lock().unwrap();
    match queue.add_item(user.into_inner(), &mut mutex_server) {
        Ok(_) => HttpResponse::Ok().body("user joining"),
        Err(e) => HttpResponse::Ok().body(format!("{}", e)),
    }
}

#[post("/leave")]
pub async fn user_leave_queue(
    user: web::Json<UserQuery>,
    queue_data: web::Data<Mutex<QueueStruct>>,
    servers_data: web::Data<Mutex<Servers>>,
) -> impl Responder {
    let queue_mutex_data = &queue_data.into_inner();
    let mut queue = queue_mutex_data.lock().unwrap();
    match queue.remove_item() {
        Ok(_) => HttpResponse::Ok().body(format!("user leaving: {:?}", user)),
        Err(e) => HttpResponse::Ok().body(format!("{}", e)),
    }
}

#[derive(Deserialize, Debug, Clone)]
struct QueryParams {
    queue_pos: usize,
    teller_id: String,
}

#[get("/time")]
async fn show_user_waiting_time(
    user_query: web::Query<QueryParams>,
    queue_data: web::Data<Mutex<QueueStruct>>,
) -> impl Responder {
    let user = user_query.into_inner().clone();
    let teller = data_source::db_actions::find_teller(user.clone().teller_id);

    match teller {
        Ok(teller_data) => {
            let timer = queue_data.lock().unwrap().get_waiting_time(teller_data, 0.0, user.queue_pos);
            HttpResponse::Ok().body(format!("Timer: {}", timer))
        }
        Err(_) => {
            HttpResponse::Ok().body("Unable to find user")
        }
    }

}

#[get("/pending")]
pub async fn show_user_pending(
    user_query: web::Query<UserQuery>,
    queue_data: web::Data<Mutex<QueueStruct>>,
) -> impl Responder {
    // let user = user_query.into_inner();
    // let timer = queue_data.lock().unwrap().get_waiting_time(user.account_number);
    // for i in (0..timer).rev() {
    //     thread::sleep(Duration::from_secs(1));
    // }
    HttpResponse::Ok().body(format!("User: "))
}
