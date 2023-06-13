use crate::data::models::{Teller, User};
use crate::data::schema::teller::active;
use crate::data::{models::Transaction, schema::transaction};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{Connection, MysqlConnection};
use dotenvy::dotenv;
use std::env;
use crate::data::schema::teller::dsl::teller;
use crate::data::schema::users::dsl::users;

fn establish_conn() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url).expect("Unable to connect to DB")
}
pub fn add_transaction(transaction_data: Transaction) -> Result<usize, Error> {
    let conn = &mut establish_conn();
    let insert_transaction = conn.transaction(|conn| {
        diesel::insert_into(transaction::table)
            .values(transaction_data)
            .execute(conn)
    });
    insert_transaction
}
pub fn find_user(user_id: String) -> Result<User, Error> {
    let conn = &mut establish_conn();
    let transactions_data = conn.transaction(|connection| {
        let results = users
            .select(User::as_select())
            .find(user_id)
            .first(connection);
        results
    });
    transactions_data
}
pub fn find_teller(teller_id: &String) -> Result<Teller, Error> {
    let conn = &mut establish_conn();
    let transactions_data = conn.transaction(|connection| {
        teller
            .select(Teller::as_select())
            .find(teller_id)
            .first(connection)
    });
    transactions_data
}
pub fn set_teller_status(status: bool, teller_id: String) -> Result<usize, Error> {
    let conn = &mut establish_conn();
    conn.transaction(|connection| {
        diesel::update(teller.find(teller_id))
            .set(active.eq(true))
            .execute(connection)
    })
}
