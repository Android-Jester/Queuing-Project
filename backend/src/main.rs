use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::web;
use actix_web::{App, HttpServer};
use queuing_server::data::models::*;
use queuing_server::interface::teller_interface::*;
use queuing_server::interface::user_interface::*;



/// Main File for runnning server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(Mutex::new(
                queuing_server::data_source::queuing_techniques::QueueStruct::<UserQuery>::new(),
            )))
            .app_data(web::Data::new(Mutex::new(Servers::new())))
            .service(server_trial)
            /*Teller Actions*/
            .configure(teller_config)
            /*User Actions*/
            .configure(user_config)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
