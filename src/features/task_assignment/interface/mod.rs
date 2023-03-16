use actix_web::{guard, web, HttpResponse, Responder};
use log::info;

use crate::features::task_assignment::controller::{remove_task, task_creation, update_task};
use crate::features::task_assignment::model::Tasks;

pub fn task_interface_config(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope("/task")
            .route(
                "/create",
                web::post()
                    .guard(guard::Header("Content-Type", "application/json"))
                    .to(create_task),
            )
            .route("/delete", web::delete().to(delete_task))
            .route("/modify", web::post().to(modify_task))
            .route("/get_all", web::get().to(get_all))
            .route("/get_completed", web::get().to(get_completed))
            .route("/get_incomplete", web::get().to(get_incomplete))
            .route("/get_owned", web::get().to(get_owned)),
    );
}

async fn create_task(data: web::Json<Tasks>) -> impl Responder {
    info!("create called");
    match task_creation(data.into_inner()) {
        Ok(_) => HttpResponse::Ok().body("Task Created Successfully"),
        Err(failed) => HttpResponse::BadRequest()
            .body(format!("Task failed to be created, reason: {}", failed)),
    }
}

async fn modify_task(data: web::Json<Tasks>) -> impl Responder {
    match update_task(data.into_inner()) {
        Ok(_) => HttpResponse::Ok().body("Task modified Successfully"),
        Err(failed) => HttpResponse::BadRequest()
            .body(format!("Task failed to be modified, reason: {}", failed)),
    }
}

async fn delete_task(id: web::Json<i32>) -> impl Responder {
    info!("delete called");
    match remove_task(id.into_inner()) {
        Ok(_) => HttpResponse::Ok().body("Task modified Successfully"),
        Err(failed) => HttpResponse::BadRequest()
            .body(format!("Task failed to be modified, reason: {}", failed)),
    }
}

async fn get_all() -> impl Responder {
    info!("Get_all called");
    HttpResponse::Ok().body("Empty")
}

async fn get_completed() -> impl Responder {
    info!("Get_completed called");
    
    HttpResponse::Ok().body("Empty")
}

async fn get_incomplete() -> impl Responder {
    info!("Get_incomplete called");
    HttpResponse::Ok().body("Empty")
}

async fn get_owned() -> impl Responder {
    info!("Get_owned called");
    HttpResponse::Ok().body("Empty")
}
