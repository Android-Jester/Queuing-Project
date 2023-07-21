pub mod actions;
pub mod auth;
pub mod deletables;
pub mod server_event;

pub mod prelude {
    pub use super::actions::*;
    pub use super::auth::*;
    pub use super::deletables::*;
    pub use super::server_event::*;
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
