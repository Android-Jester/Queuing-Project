use crate::features::task_assignment::controller::task_creation;
use crate::features::task_assignment::model::Tasks;
use actix_web::{post, web, HttpResponse, Responder};
#[post("/create-task")]
pub async fn create_task(data: web::Json<Tasks>) -> impl Responder {
    match task_creation(data.into_inner()) {
        Ok(_) => HttpResponse::Ok().body("Task Created Successfully"),
        Err(failed) => HttpResponse::BadRequest()
            .body(format!("Task failed to be created, reason: {}", failed)),
    }
}

#[post("/modify_task")]
pub async fn modify_task(data: web::Json<Tasks>) -> impl Responder {
    match update_task(data.into_inner()) {
        Ok(_) => HttpResponse::Ok().body("Task modified Successfully"),
        Err(failed) => HttpResponse::BadRequest()
            .body(format!("Task failed to be modified, reason: {}", failed)),
    }
}
