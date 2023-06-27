pub mod models;
pub mod user_interface;

pub mod prelude {
    pub use super::models::*;
    pub use super::user_interface::*;
    use crate::prelude::*;

    pub fn user_config(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/user")
                .service(user_login)
                .service(guest_login)
                .service(join_queue)
                .service(leave_queue),
        );
    }

    #[get("/")]
    pub async fn list_users() -> impl Responder {
        match list_users_db() {
            Ok(users) => HttpResponse::Ok().json(users),
            Err(err) => HttpResponse::BadRequest().body(err),
        }
    }
}
