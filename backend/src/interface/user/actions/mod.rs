use crate::prelude::*;
#[derive(Serialize, Deserialize, Clone)]
pub struct JoinQuery {
    pub national_id: String,
    pub activity: String,
}

/// Users and guests can join the main queue and assigned tellers
#[get("/join")]
pub async fn main_queue_join(
    // user_input: Json<UserInputData>,
    user_input: Query<ClientInputData>,
    main_queue: Data<Mutex<MainQueue>>,
    sub_queues: Data<Mutex<SubQueues>>,
    server_broadcast: Data<ServerBroadcaster>,
    client_broadcast: Data<ClientBroadcaster>,
) -> impl Responder {
    let national_id = user_input.national_id.clone();
    let user_name = db_find_user(national_id.clone()).unwrap().name;
    let main_queues = main_queue.into_inner();
    let sub_queues = sub_queues.into_inner();
    let server_broadcast = server_broadcast.into_inner();
    let client_broadcast = client_broadcast.into_inner();
    let mut sub_queue = sub_queues.lock();
    let mut main_queue = main_queues.lock();
    let user_queue_input = user_input.into_inner();
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
        ClientQueueData::new(user_queue_input, user_name, 0),
        &mut sub_queue,
    ) {
        info!("SERVER QUEUE: {:?}", sub_queue);
        let added_user = added_user;

        client_broadcast.new_client(&added_user, national_id, &mut sub_queue, client_broadcast.clone(), server_broadcast.clone()).await
    } else {
        client_broadcast.error().await
    }
}



/// Removes user from the queue and resets the queue
#[post("/leave")]
pub async fn main_queue_leave(
    user: Json<JoinQuery>,
    main_queues: Data<Mutex<MainQueue>>,
    sub_queues: Data<Mutex<SubQueues>>,
    server_broadcaster: Data<ServerBroadcaster>,
) -> impl Responder {
    let user = user.into_inner();
    let sub_queues = sub_queues.into_inner();
    let main_queues = main_queues.into_inner();
    let mut main_queue = main_queues.lock();
    let mut sub_queue = sub_queues.lock();
    let removed_user_teller = main_queue.user_remove(user.national_id, &mut sub_queue);
    match removed_user_teller {
        Ok(teller_loc) => {
            server_broadcaster
                .user_update(&mut sub_queue, teller_loc)
                .await;
            HttpResponse::Ok().body(format!("Removed: USER"))
        }
        Err(err) => HttpResponse::NotFound().body(err),
    }
}
