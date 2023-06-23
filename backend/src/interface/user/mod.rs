pub mod user_interface;
pub mod models;
use user_interface::*;
use actix_web::web;
pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(login_user_request)
            .service(login_guest_request)
            .service(user_join_queue)
            .service(user_leave_queue)
            // .service(
            //     web::scope("/")
            //         .guard(guard::Header("content-type", "text/event-stream"))
            //         .guard(guard::Header("cache-control", "no-cache")), // .service(show_user_waiting_time),
            // ),
    );
}
