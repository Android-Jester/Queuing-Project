pub mod data;
pub mod data_source;
#[cfg(test)]
pub mod tests;
use crate::{
    data::models::{Teller, Transaction, User},
    data_source::db_actions::add_transaction,
};
use actix_web::{get, post, web, HttpResponse, Responder};
use crate::data_source::db_actions::set_teller_status;
use crate::data_source::queuing_techniques::{add_user_queue, remove_user_queue};


#[derive(Serialize, Deserialize, Debug)]
struct LoginData {
    account_number: String,
    password: String,
}





/*User Space*/

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




#[post("/user/join")]
pub async fn user_join_queue(user: web::Json<User>) -> impl Responder {
    add_user_queue(user.into_inner(), );
    HttpResponse::Ok().body("user leaving")
}

#[get("/user/waiting_time")]
pub async fn show_user_waiting_time(user: web::Json<User>) -> impl Responder {
    // let user = user.into_inner();
    // let timer = get_waiting_time(user.account_number);
    // for i in (0..timer).rev() {
    //     thread::sleep(Duration::from_secs(1));
    // }
    HttpResponse::Ok().body("Hello")
}

/*Tellers Space*/

#[post("/teller/add_transaction")]
pub async fn record_transaction(transaction: web::Json<Transaction>) -> impl Responder {
    let transaction_data = transaction.into_inner();
    match add_transaction(transaction_data) {
        Ok(d) => HttpResponse::Ok().body(format!("Done: {}", d)),
        Err(_) => HttpResponse::BadRequest().body("Unable to accept data"),
    }
}





// #[get("/teller/server_list")]
// pub async fn teller_list(id: String) -> impl Responder {
//     HttpResponse::Ok().body("Accepted")
// }

#[post("/teller/status")]
pub async fn change_teller_status(
    teller_status: web::Query<bool>,
    teller_id: web::Query<String>,
) -> impl Responder {
    let teller_status = set_teller_status(teller_status.into_inner(), teller_id.into_inner());
    HttpResponse::Ok().body(format!("{}", teller_status.unwrap()))
}

#[post("/teller/logout")]
pub async fn logout_teller(teller: web::Json<Teller>) -> impl Responder {
    HttpResponse::Ok().body(format!("Logged out"))
}


#[post("/teller/remove")]
pub async fn teller_remove_user() -> impl Responder {
    remove_user_queue();
    HttpResponse::Ok().body("user leaving")
}
