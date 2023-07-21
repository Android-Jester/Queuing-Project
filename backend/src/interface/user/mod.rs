pub mod activities;
pub mod sse;
pub mod prelude {
    use crate::prelude::db_list_users;

    pub use super::activities::*;
    pub use super::sse::*;
    use crate::prelude::*;
    pub fn user_config(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/user")
                .service(user_login)
                .service(guest_login)
                .service(list_users)
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
