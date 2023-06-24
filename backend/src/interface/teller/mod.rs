pub mod teller_interface;
pub mod teller_queue;

pub mod prelude {
    pub use super::teller_interface::*;
    pub use super::teller_queue::*;
    use actix_web::web;

    pub fn teller_config(conf: &mut web::ServiceConfig) {
        conf.service(
            web::scope("/teller")
                .service(record_transaction)
                .service(change_teller_status)
                .service(login_teller_request)
                .service(remove_user)
                .service(user_queues)
                .service(logout_teller),
        );
    }
}
