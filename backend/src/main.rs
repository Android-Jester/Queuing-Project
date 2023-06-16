use std::sync::Mutex;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use queues::{Queue, queue};
use queuing_server::data::models::UserQuery;
use queuing_server::interface::teller_interface::*;
use queuing_server::interface::user_interface::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            /*Teller Actions*/
            .service(
                web::scope("/teller")
                    .service(record_transaction)
                    .service(change_teller_status)
                    .service(login_teller_request)
                    .service(remove_user)
                    .service(logout_teller)
                    .service(queue_show)
            )
            /*User Actions*/
            .service(
                web::scope("/user")
                    .app_data(
                        web::Data::new(Mutex::new(queuing_server::data_source::queuing_techniques::QueueStruct::<UserQuery>::new()))
                    )
                    .service(login_user_request)
                    .service(login_guest_request)
                    .service(user_join_queue)
                    .service(user_leave_queue)
                    .service(show_user_waiting_time)
            )
    }
    )
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}


