use crate::data::models::{Teller, User};
use crate::data::schema::teller::active;
use crate::data::{models::Transaction, SERVER_COUNT};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{Connection, MysqlConnection};
use dotenvy::dotenv;
use std::env;
use crate::data::schema::guests::national_id;
use crate::data::schema::teller::dsl::teller;
use crate::data::schema::transaction::dsl::transaction;
use crate::data::schema::users::dsl::users;

fn establish_conn() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url).expect("Unable to connect to DB")
}
pub fn add_transaction(transaction_data: Transaction) -> Result<usize, Error> {
    let conn = &mut establish_conn();
    let insert_transaction = conn.transaction(|conn| {
        diesel::insert_into(transaction)
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
            .set(active.eq(status))
            .execute(connection)
    })
}
pub fn get_all_service_times() {
    let conn = &mut establish_conn();
    let mut data: Vec<f32> = Vec::new();
    let mut server_loc: Vec<i32> = Vec::new();
    let transactions_data = conn.transaction(|connection| {
        let results = transaction
            .select(Transaction::as_select())
            .load(connection);
        results
    });
    match transactions_data {
        Ok(transactions) => {
            for events in transactions {
                data.push(events.duration);

                let teller_data = conn.transaction(|connection| {
                    let results = teller
                        .select(Teller::as_select())
                        .find(events.server_id)
                        .first(connection);
                    results
                });
                server_loc.push(teller_data.expect("Unable get teller").server_station)
            }

        }
        Err(_) => {}
    };
}
pub fn login_user(
    conn: &mut MysqlConnection,
    acc_num: &String,
    pass_word: &String,
) -> Option<User> {
    // let db_data = conn.transaction(|conn| {});
    let user = users::table
        .filter(national_id.eq(acc_num))
        .select(User::as_select())
        .first(conn);

    match user {
        Ok(user_data) => {
            if user_data.password.eq(pass_word) {
                Some(user_data)
            } else {
                None
            }
        }
        Err(_) => None,
    }
    // let data = db_data.expect("Unable to search users");
    // data
}
pub fn login_guest(
    conn: &mut MysqlConnection,
    account_numbers: String,
    passwords: String,
) -> usize {
    let new_user = User {
        account_number: account_numbers,
        password: passwords,
    };

    conn.transaction(|conn| {
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)
    })
        .expect("Error loading Data")
}

// Return a vec of a vec of durations and position
fn sort_tellers(transactions: Vec<Transaction>, teller_data: &Teller) {
    // let mut data = [0f32; SERVER_COUNT];
let mut data = Vec::new();
    // Depending on the server location in the queue split the duration accordingly
    for transaction_data in transactions {
        data[teller_data.server_station as usize] = transaction_data.duration;
    }
}
