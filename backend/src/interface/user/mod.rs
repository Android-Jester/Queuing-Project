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
                .service(main_queue_join)
                .service(main_queue_leave),
        );
    }

    #[get("/")]
    pub async fn list_users() -> impl Responder {
        match db_list_users() {
            Ok(users) => HttpResponse::Ok().json(users),
            Err(err) => HttpResponse::BadRequest().body(err),
        }
    }
}
