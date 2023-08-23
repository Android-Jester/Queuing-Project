use crate::prelude::*;

#[get("")]
pub async fn tellers_listing() -> impl Responder {
    match db_list_tellers() {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}
