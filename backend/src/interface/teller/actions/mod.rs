use crate::prelude::*;
#[post("/dismiss")]
pub async fn record_transaction(
    transaction: Json<Transaction>,
    queue_data: Data<Mutex<MainQueue>>,
    sub_queue_data: Data<Mutex<SubQueues>>,
) -> impl Responder {
    let transaction = transaction.into_inner();
    match add_transaction(transaction.clone()) {
        Ok(_) => {
            match queue_data
                .lock()
                .user_dismiss(transaction.client_national_id, &mut sub_queue_data.lock())
            {
                Ok(_) => {
                    info!("Transaction Recorded");
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

#[post("/remove")]
pub async fn remove_user(
    national_id: Query<RemoveUserQuery>,
    queue_data: Data<Mutex<MainQueue>>,
    server_queue: Data<Mutex<SubQueues>>,
) -> impl Responder {
    let mut queue = queue_data.lock();
    let user = queue.search_user(national_id.national_id.clone());
    match queue.user_remove(user, &mut server_queue.lock()) {
        Ok(_) => {
            info!("User Removed");
            HttpResponse::Ok().body("User Removed")
        }
        Err(e) => {
            error!("ERROR: {}", e);
            HttpResponse::NotFound().body(e)
        }
    }
}

#[derive(Deserialize)]
pub struct ServerQueueQuery {
    teller_loc: usize,
}

#[get("/queue")]
pub async fn user_queues(
    req: HttpRequest,
    server_queues: Data<Mutex<SubQueues>>,
    teller_loc: Query<ServerQueueQuery>,
    server_broadcaster: Data<ServerBroadcaster>,
) -> impl Responder {
    let queue = server_queues.lock();
    let json_data = queue.teller_show_queue(teller_loc.teller_loc);
    info!("Data: {:?}", json_data);
    server_broadcaster
        .new_client(req.peer_addr().unwrap().ip().to_string(), &json_data)
        .await
}
