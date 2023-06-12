use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use auth::{establish_connection, login_user};
use serde::{Deserialize, Serialize};

use std::fmt::Debug;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new().service(login).service(starter), // .service(signup)
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn starter() -> impl Responder {
    HttpResponse::Ok().body("Started")
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginData {
    account_number: String,
    password: String,
}

#[post("/login")]
async fn login(login_data: web::Json<LoginData>) -> impl Responder {
    let conn = &mut establish_connection();
    let user_data = login_user(conn, &login_data.account_number, &login_data.password);
    if let Some(user) = user_data {
        HttpResponse::Accepted().body("Logged In")
    } else {
        HttpResponse::NotFound().body("User Not Found")
    }
}

#[cfg(test)]
mod test;
