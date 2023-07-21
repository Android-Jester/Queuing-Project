pub mod actions;
pub mod auth;
pub mod client_event;
pub mod deletable;

pub mod prelude {
    pub use super::actions::*;
    pub use super::auth::*;
    pub use super::client_event::*;
    pub use super::deletable::*;
    use crate::prelude::*;
    pub fn user_config(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/user")
                .service(user_login)
                .service(guest_login)
                .service(client_listings)
                .service(show_countdowner)
                .service(main_queue_join)
                .service(main_queue_leave),
        );
    }
}
