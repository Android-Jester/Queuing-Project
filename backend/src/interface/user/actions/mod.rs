use crate::prelude::*;
#[derive(Serialize, Deserialize, Clone)]
pub struct JoinQuery {
    pub national_id: String,
    pub activity: String,
}

/// Users and guests can join the main queue and assigned tellers
#[get("/join")]
pub async fn main_queue_join(
    user_input: Query<ClientInputData>,
    main_queue: Data<Mutex<Queue>>,
    sub_queues: Data<Mutex<SubQueues>>,
    server_broadcast: Data<ServerBroadcaster>,
    client_broadcast: Data<ClientBroadcaster>,
) -> Either<HttpResponse, actix_web_lab::sse::Sse<actix_web_lab::sse::ChannelStream>> {
    let user_input = user_input.into_inner();
    let national_id = user_input.national_id;

    // Queue Structs
    let main_queues = main_queue.into_inner();
    let sub_queues = sub_queues.into_inner();

    // Broadcasters
    let server_broadcast = server_broadcast.into_inner();
    let client_broadcast = client_broadcast.into_inner();
    let mut sub_queue = sub_queues.lock();
    let mut main_queue = main_queues.lock();
    let mut tellers_service_times: [f64; SERVER_COUNT as usize] = [0.0; SERVER_COUNT as usize];
    let _ = sub_queue.tellers.iter_mut().map(|data| {
        tellers_service_times[data.teller.station as usize] =
            (data.teller.service_time / 60) as f64;
    });
    let prediction = prediction(tellers_service_times) as usize % sub_queue.teller_count();
    // let prediction = 0;
    let user_input = ClientInputData {
        activity: user_input.activity.clone(),
        national_id: national_id.clone(),
    };
    info!("Pred: {:?}", prediction);
    let queue_count = get_current_queue_count();
    // if queue_count < CUSTOMER_COUNT as usize {
    if prediction < sub_queue.teller_count() {
        let teller_id = &sub_queue.tellers[prediction].teller.server_id;
        if let Ok(mut added_user) = main_queue.user_add(
            ClientQueueData::new(
                user_input.clone(),
                teller_id.clone(),
                db_find_user(national_id.clone()).unwrap().name,
                prediction as i32,
            ),
            &mut sub_queue,
        ) {
            let user = &mut added_user;
            info!("Catches");
            Either::Right(
                client_broadcast
                    .new_client(
                        national_id,
                        user,
                        &mut sub_queue,
                        user.server_location as usize,
                        server_broadcast.clone(),
                    )
                    .await,
            )
        } else {
            Either::Left(HttpResponse::BadRequest().body("Teller Not available"))
        }
        // } else {
        // Either::Left(HttpResponse::BadRequest().body("Teller Not available"))
        // }
    } else if queue_count == 0 {
        Either::Left(HttpResponse::BadRequest().body("Queue Not available"))
    } else {
        Either::Left(HttpResponse::BadRequest().body("Queue is full"))
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NationalId {
    pub national_id: String,
}

/// Removes user from the queue and resets the queue
#[post("/leave")]
pub async fn main_queue_leave(
    user: Json<NationalId>,
    main_queues: Data<Mutex<Queue>>,
    sub_queues: Data<Mutex<SubQueues>>,
    server_broadcaster: Data<ServerBroadcaster>,
    client_broadcaster: Data<ClientBroadcaster>,
) -> impl Responder {
    let user = user.into_inner();
    let sub_queues = sub_queues.into_inner();
    let main_queues = main_queues.into_inner();
    let mut main_queue = main_queues.lock();
    let mut sub_queue = sub_queues.lock();
    let removed_user_teller = main_queue
        .user_remove(
            user.national_id,
            &mut sub_queue,
            client_broadcaster.into_inner(),
        )
        .await;
    match removed_user_teller {
        Ok(removed_client) => {
            server_broadcaster
                .user_update(&sub_queue, removed_client.server_location as usize)
                .await;
            HttpResponse::Ok().body(format!("Removed: {}", removed_client.national_id))
        }
        Err(err) => HttpResponse::Conflict().body(err),
    }
}
