use crate::data::models::{self, TellerQuery};
use crate::data::models::{Teller, TellerLogin, Transaction, UserInsert, UserQuery};
use crate::data::schema::{Guests, Tellers, Transactions, Users};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{Connection, MysqlConnection};
use dotenvy::dotenv;
use std::env;

pub fn establish_conn() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url).expect("Unable to connect to DB")
}

/*Adding Data*/
pub fn add_transaction(transaction_data: Transaction) -> Result<usize, Error> {
    let conn = &mut establish_conn();
    conn.transaction(|conn| {
        diesel::insert_into(Transactions::dsl::Transactions)
            .values(transaction_data)
            .execute(conn)
    })
}
pub fn register_user(user_insert_data: UserInsert) -> Result<usize, Error> {
    let conn = &mut establish_conn();
    conn.transaction(|conn| {
        diesel::insert_into(Users::dsl::Users)
            .values(user_insert_data)
            .execute(conn)
    })
}
pub fn register_teller(teller_insert_data: models::TellerInsert) -> Result<usize, Error> {
    let conn = &mut establish_conn();
    conn.transaction(|conn| {
        diesel::insert_into(Tellers::dsl::Tellers)
            .values(teller_insert_data)
            .execute(conn)
    })
}
pub fn register_guest(insert_data: models::GuestInsert) -> Result<usize, Error> {
    let conn = &mut establish_conn();
    conn.transaction(|conn| {
        diesel::insert_into(Guests::dsl::Guests)
            .values(insert_data)
            .execute(conn)
    })
}

/// Obtain all service Times and sort them according to the randomforest model
pub fn list_transactions() -> Result<Vec<Transaction>, &'static str> {
    let conn = &mut establish_conn();
    let transactions_data = conn.transaction(|connection| {
        Transactions::dsl::Transactions
            .select(Transaction::as_select())
            .load(connection)
    });

    match transactions_data {
        Ok(transactions) => Ok(transactions),
        Err(_) => Err("Unable to find transactions"),
    }
}
pub fn list_users_db() -> Result<Vec<UserInsert>, &'static str> {
    let conn = &mut establish_conn();
    let transactions_data = conn.transaction(|connection| {
        Users::dsl::Users
            .select(UserInsert::as_select())
            .load(connection)
    });

    match transactions_data {
        Ok(transactions) => Ok(transactions),
        Err(_) => Err("Unable to find transactions"),
    }
}

/*Authentication */
pub fn login_user(login_data: models::UserLogin) -> Result<String, &'static str> {
    let conn = &mut establish_conn();
    let user_data = conn.transaction(|connection| {
        Users::dsl::Users
            .select(models::UserLoginQuery::as_select())
            .filter(Users::account_number.eq(login_data.account_number))
            .first(connection)
    });
    match user_data {
        Ok(user) => {
            if user.password.eq(&login_data.password) {
                Ok(user.national_id)
            } else {
                Err("Unable to login User")
            }
        }
        Err(_) => Err("Unable to Find User"),
    }
}
pub fn login_guest(guest: models::Guest) -> Result<String, &'static str> {
    let conn = &mut establish_conn();
    let res = conn.transaction(|conn| {
        diesel::insert_into(Guests::table)
            .values(&guest)
            .execute(conn)
    });
    match res {
        Ok(_) => Ok(guest.national_id),
        Err(_) => Err("Guest cannot be logged in"),
    }
}
pub fn login_teller(teller_login: TellerLogin) -> Result<(String, i32), &'static str> {
    let conn = &mut establish_conn();
    let transactions_data = conn.transaction(|connection| {
        Tellers::dsl::Tellers
            .select(TellerQuery::as_select())
            .find(teller_login.server_id)
            .first(connection)
    });
    match transactions_data {
        Ok(teller) => {
            if teller.password.eq(&teller_login.password) {
                Ok((teller.server_id, teller.server_station))
            } else {
                Err("Unable to login Teller")
            }
        }
        Err(_) => Err("Unable to Find Teller"),
    }
}

pub fn find_teller(teller_id: String) -> Result<Teller, &'static str> {
    let conn = &mut establish_conn();
    let transactions_data = conn.transaction(|connection| {
        Tellers::dsl::Tellers
            .select(Teller::as_select())
            .find(teller_id)
            .first(connection)
    });
    match transactions_data {
        Ok(teller) => Ok(teller),
        Err(_) => Err("Unable to Find Telle"),
    }
}

pub fn find_teller_id(teller_id: i32) -> Result<Teller, &'static str> {
    let conn = &mut establish_conn();
    let transactions_data = conn.transaction(|connection| {
        Tellers::dsl::Tellers
            .select(Teller::as_select())
            .filter(Tellers::server_station.eq(teller_id))
            .first(connection)
    });
    match transactions_data {
        Ok(teller) => Ok(teller),
        Err(_) => Err("Unable to Find Telle"),
    }
}

pub fn find_user(user_national_id: String) -> Result<UserQuery, &'static str> {
    let conn = &mut establish_conn();
    let transactions_data = conn.transaction(|connection| {
        Users::dsl::Users
            .select(UserQuery::as_select())
            .filter(Users::national_id.eq(user_national_id))
            .first(connection)
    });
    match transactions_data {
        Ok(teller) => Ok(teller),
        Err(_) => Err("Unable to Find Teller"),
    }
}
/* Teller Actions */
pub fn set_teller_status(status: bool, teller_id: String) -> Result<usize, Error> {
    let conn = &mut establish_conn();
    conn.transaction(|connection| {
        diesel::update(Tellers::dsl::Tellers.find(teller_id))
            .set(Tellers::active.eq(status))
            .execute(connection)
    })
}
