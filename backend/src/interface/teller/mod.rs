pub mod activities;
pub mod sse;
pub mod prelude {
    pub use super::activities::*;
    pub use super::sse::*;
    use crate::prelude::*;
    pub fn teller_config(conf: &mut ServiceConfig) {
        conf.service(
            scope("/teller")
                .service(record_transaction)
                .service(login_teller)
                .service(remove_user)
                .service(user_queues)
                .service(tellers_listing)
                .service(logout_teller),
        );
    }
}
