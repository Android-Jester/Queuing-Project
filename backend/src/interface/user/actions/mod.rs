use crate::prelude::*;
#[derive(Serialize, Deserialize)]
pub struct JoinQuery {
    pub national_id: String,
    pub activity: String,
}

/// Users and guests can join the main queue and assigned tellers
#[get("/join")]
pub async fn main_queue_join(
    // user_input: Json<UserInputData>,
    req: HttpRequest,
    user_input: Query<ClientInputData>,
    main_queue: Data<Mutex<MainQueue>>,
    sub_queues: Data<Mutex<SubQueues>>,
    server_broadcast: Data<ServerBroadcaster>,
    client_broadcast: Data<ClientBroadcaster>,
) -> impl Responder {
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
    if let Ok(added_user) = main_queue.user_add(
        ClientQueueData::new(user_input.into_inner(), user_name, 0),
        &mut sub_queue,
    ) {
        server_broadcast.user_update(&sub_queue, 0).await;
        let ip = req.peer_addr().unwrap().ip().to_string();

        client_broadcast.new_client(&added_user, ip).await
    } else {
        client_broadcast.error().await
    }
}

#[get("/updatable")]
pub async fn show_countdowner(
    query: Query<ClientInputData>,
    req: HttpRequest,
    main_queue: Data<Mutex<MainQueue>>,
    sub_queue: Data<Mutex<SubQueues>>,
    client_broadcaster: Data<ClientBroadcaster>,
) -> impl Responder {
    let ip = req.peer_addr().unwrap().ip();
    let mut sub_queue = sub_queue.lock();
    let user = main_queue.lock().search_user(query.national_id.clone());
    sub_queue
        .timer_countdown(
            ip.to_string(),
            user.sub_queue_position,
            user.service_location,
            client_broadcaster,
        )
        .await;
    HttpResponse::Ok()
}

/// Removes user from the queue and resets the queue
#[post("/leave")]
pub async fn main_queue_leave(
    user: Json<ClientQueueData>,
    main_queue: Data<Mutex<MainQueue>>,
    sub_queue: Data<Mutex<SubQueues>>,
    server_broadcaster: Data<ServerBroadcaster>,
) -> impl Responder {
    let user = user.into_inner();
    let mut main_queue = main_queue.lock();
    let mut sub_queue = sub_queue.lock();
    let removed_user = main_queue.user_remove(user.clone(), &mut sub_queue);
    match removed_user {
        Ok(removed_user) => {
            server_broadcaster
                .user_update(&sub_queue, removed_user.service_location)
                .await;
            HttpResponse::Ok().body(format!("Removed: {:?}", removed_user))
        }
        Err(err) => HttpResponse::NotFound().body(err),
    }
}
