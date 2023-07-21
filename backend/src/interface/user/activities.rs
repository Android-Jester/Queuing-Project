use crate::prelude::*;
/// Allows registered users to use the queuing service
#[post("/login")]
pub async fn user_login(login_data: Json<UserLogin>) -> impl Responder {
    match db_login_user(login_data.into_inner()) {
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
pub async fn guest_login(guest: Json<GuestQuery>) -> impl Responder {
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

#[derive(Serialize, Deserialize)]
pub struct JoinQuery {
    pub national_id: String,
    pub activity: String,
}

/// Users and guests can join the main queue and assigned tellers
#[get("/join")]
pub async fn main_queue_join(
    // user_input: Json<UserInputData>,
    user_input: Query<UserInputData>,
    main_queue: Data<Mutex<MainQueue>>,
    sub_queues: Data<Mutex<SubQueues>>,
    broadcast_agent: Data<Broadcaster>,
    broadcaster_agent2: Data<BroadcasterUser>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    // let sub_queue = sub_queues.lock().unwrap();
    let peer_id = req.peer_addr().unwrap().ip().to_string();
    let user_name = db_find_user(user_input.national_id.clone()).unwrap().name;
    let mut sub_queue = sub_queues.lock();
    let mut main_queue = main_queue.lock();
    // let mut tellers_service_times:[f64; SERVER_COUNT] = [0.0; SERVER_COUNT];
    // let _ = sub_queue.tellers.iter().map(|data| {
    //     tellers_service_times[data.teller.server_station as usize] = data.teller.service_time.as_secs_f64() / 60.0;
    // });
    // let prediction = prediction(tellers_service_times) as usize;
    // let user_input = UserInputData {
    //     activity: query.activity.clone(),
    //     national_id: query.national_id.clone(),
    // };
    info!("User Data: {:?}", user_input);
    if let Ok(added_user) = main_queue.user_add(
        UserQueuePos::new(user_input.into_inner(), user_name, 0),
        &mut sub_queue,
    ) {
        info!("Successful Join");
        broadcast_agent.broadcast_users(&sub_queue, 0).await;
        info!("DDDD: {:?}", added_user);
        broadcaster_agent2
            .new_client(&added_user, peer_id)
            // &mut sub_queue, &broadcaster_agent2)
            .await
        // HttpResponse::Ok().body("Hello Stream H")
    } else {
        info!("Hello");
        broadcaster_agent2.error().await
        // HttpResponse::Ok().body("Hello Stream H")
    }
}

#[get("/updatable")]
pub async fn show_countdowner(
    query: Query<UserInputData>,
    queue: Data<Mutex<SubQueues>>,
    main_queue: Data<Mutex<MainQueue>>,
    broadcaster_agent2: Data<BroadcasterUser>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    let peer_id = req.peer_addr().unwrap().ip().to_string();
    let mut main_queue = main_queue.lock();
    let mut user_queue = queue.lock();
    let user = main_queue.search_user(query.national_id.clone()).unwrap();
    warn!("USER BEFORE LOOP: {:?}", user);
    user_queue
        .timer_countdown(
            peer_id,
            user.sub_queue_position,
            user.service_location,
            broadcaster_agent2,
        )
        .await;

    HttpResponse::Ok()
}

/// Removes user from the queue and resets the queue
#[post("/leave")]
pub async fn main_queue_leave(
    user: Json<UserQueuePos>,
    main_queue: Data<Mutex<MainQueue>>,
    sub_queue: Data<Mutex<SubQueues>>,
    broadcast_agent: Data<Broadcaster>,
) -> impl Responder {
    info!("Attempted leaving: {:?}", user);
    let user = user.into_inner();
    let mut main_queue = main_queue.lock();
    let mut sub_queue = sub_queue.lock();
    let removed_user = main_queue.user_remove(user.clone(), &mut sub_queue);
    match removed_user {
        Ok(removed_user) => {
            broadcast_agent
                .broadcast_users(&sub_queue, removed_user.service_location)
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
