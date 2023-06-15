use std::sync::Mutex;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use queues::{Queue, queue};
use queuing_server::*;
use queuing_server::data::models::UserQuery;
use queuing_server::data_source::db_actions::add_transaction;
use queuing_server::data_source::queuing_techniques::QueueStruct;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let user_queue = Mutex::new(QueueStruct::<UserQuery>::new(queue![]));
        App::new()
            /*Authentication*/
            .service(login_user_request)
            .service(login_guest_request)
            .service(login_teller_request)
            /*Teller Actions*/
            .service(change_teller_status)
            .service(logout_teller)
            .service(remove_user)
            .service(record_transaction)
            .service(queue_show)

            /*Queue Actions*/
            .service(logout_teller)
            .service(teller_remove_user)

            /*User Actions*/
            .service(
                web::scope("/user")
                    .app_data(
                        web::Data::new(user_queue)
                    )
                    .route("/join", web::post().service(user_join_queue))
                    .route("/leave", web::post().service(user_leave_queue))
                    .route("/time", web::post().service(show_user_waiting_time))
            )
    }
    )
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}


