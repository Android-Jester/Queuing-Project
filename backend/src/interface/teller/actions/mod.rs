use crate::prelude::*;

#[post("/dismiss/{teller_pos}")]
pub async fn record_transaction(
    transaction: Json<Transaction>,
    teller_pos: Path<i32>,
    queue_data: Data<Mutex<Queue>>,
    sub_queue_data: Data<Mutex<SubQueues>>,
    broadcast: Data<ClientBroadcaster>,
    server_broadcast: Data<ServerBroadcaster>,
) -> impl Responder {
    warn!("Called DATA");
    dbg!(transaction.clone());
    let mut transaction = transaction.into_inner();
    transaction.created_date = chrono::Utc::now().naive_utc();

    match add_transaction(transaction.clone()) {
        Ok(_) => {
            let service_location = teller_pos.into_inner();
            let mut subqueue = sub_queue_data.lock();
            match queue_data
                .lock()
                .user_remove(
                    transaction.client_national_id,
                    &mut subqueue,
                    broadcast.into_inner(),
                )
                .await
            {
                Ok(_) => {
                    info!("Transaction Recorded");
                    server_broadcast
                        .user_update(&subqueue, service_location as usize)
                        .await;
                    HttpResponse::Ok().body("Transaction Recorded")
                }
                Err(e) => {
                    error!("ERROR: {}", e);
                    HttpResponse::NotFound().body(e)
                }
            }
        }
        Err(_) => HttpResponse::BadRequest().body("Unable to accept data"),
    }
}

#[derive(Serialize, Deserialize)]
pub struct RemoveUserQuery {
    national_id: String,
}

#[post("/remove/{teller_pos}")]
pub async fn remove_user(
    teller_pos: Path<i32>,
    queue_data: Data<Mutex<Queue>>,
    server_queue: Data<Mutex<SubQueues>>,
    broadcast: Data<ClientBroadcaster>,
    server_broadcast: Data<ServerBroadcaster>,
    transaction: Json<CancelStruct>,
) -> impl Responder {
    let mut queue = queue_data.lock();
    let mut subqueue = server_queue.lock();
    let service_location = teller_pos.into_inner();
    let cancelled_transaction = transaction.into_inner();
    match add_cancelled(cancelled_transaction.clone()) {
        Ok(_) => {
            match queue
                .user_remove(
                    cancelled_transaction.client_national_id.clone(),
                    &mut subqueue,
                    broadcast.into_inner(),
                )
                .await
            {
                Ok(_) => {
                    info!("User Removed");
                    server_broadcast
                        .user_update(&subqueue, service_location as usize)
                        .await;
                    HttpResponse::Ok().body("User Removed")
                }
                Err(e) => {
                    error!("ERROR: {}", e);
                    HttpResponse::NotFound().body(e)
                }
            }
        }
        Err(err) => {
            error!("ERROR: {}", err);
            HttpResponse::NotFound().body(err.to_string())
        }
    }
}

#[derive(Deserialize)]
pub struct ServerQueueQuery {
    teller_position: usize,
}

#[get("/queue")]
pub async fn user_queues(
    server_queues: Data<Mutex<SubQueues>>,
    teller_loc: Query<ServerQueueQuery>,
    server_broadcaster: Data<ServerBroadcaster>,
) -> impl Responder {
    let server_queues = server_queues.into_inner().clone();
    let queue = server_queues.lock();
    // let json_data = queue.teller_show_queue(teller_loc.teller_position);
    server_broadcaster
        .new_client(&queue, teller_loc.teller_position)
        .await
}
