use crate::prelude::*;
/// Allows registered users to use the queuing service
#[post("/login")]
pub async fn user_login(login_data: Json<ClientLoginData>) -> impl Responder {
    match db_login_user(login_data.into_inner()) {
        Ok(data) => {
            info!("User: {:?} is logged in", data);
            HttpResponse::Ok().json(data)
        }
        Err(_) => {
            error!("Invalid Login");
            HttpResponse::BadRequest().body("User Not Found")
        }
    }
}

/// Allows guests to use the service
#[post("/guest/login")]
pub async fn guest_login(guest: Json<GuestQuery>) -> impl Responder {
    match db_add_guest(guest.into_inner()) {
        Ok(added_guest) => {
            info!("Guest {} has loggedin", added_guest.name);
            HttpResponse::Ok().json(added_guest)
        }
        Err(err) => {
            error!("ERROR: {}", err);
            HttpResponse::BadRequest().body(err)
        }
    }
}
