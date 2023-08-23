use crate::prelude::*;
#[get("")]
pub async fn client_listings() -> impl Responder {
    match db_list_clients() {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}
