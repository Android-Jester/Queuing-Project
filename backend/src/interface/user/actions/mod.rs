
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
    let national_id = user_input.national_id.clone();
    let user_name = db_find_user(national_id.clone()).unwrap().name;
    let main_queues = main_queue.into_inner();
    let sub_queues = sub_queues.into_inner();
    let server_broadcast = server_broadcast.into_inner();
    let client_broadcast = client_broadcast.into_inner();
    let mut sub_queue = sub_queues.lock();
    let mut main_queue = main_queues.lock();
    let mut tellers_service_times: [i32; SERVER_COUNT as usize] = [0; SERVER_COUNT as usize];
    let _ = sub_queue.tellers.iter_mut().map(|data| {
        tellers_service_times[data.teller.station as usize] = data.teller.service_time / 60;
    });
    // let prediction = prediction(tellers_service_times) as usize;
    let prediction = 0;
    let user_input = ClientInputData {
        activity: user_input.activity.clone(),
        national_id: user_input.national_id.clone(),
    };
    info!("Pred: {:?}", prediction);
    if prediction < sub_queue.teller_count() {
        let teller_id = &sub_queue.tellers[prediction].teller.server_id;
        if let Ok(mut added_user) = main_queue.user_add(
            ClientQueueData::new(user_input.clone(), teller_id.clone(), user_name, prediction as i32),
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
                        client_broadcast.clone(),
                    )
                    .await,
            )
        } else {
            Either::Left(HttpResponse::BadRequest().body("Teller Not available"))
        }
    } else {
        Either::Left(HttpResponse::BadRequest().body("Teller Not available"))
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
                .user_update(&mut sub_queue, teller_loc.server_location as usize)
                .await;
            HttpResponse::Ok().body(format!("Removed: USER"))
        }
        Err(err) => HttpResponse::Conflict().body(err),
    }
}
