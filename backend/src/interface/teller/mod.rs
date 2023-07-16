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
                .service(login_teller)
                .service(remove_user)
                .service(user_queues)
                .service(tellers_listing)
                .service(logout_teller),
        );
    }
}
