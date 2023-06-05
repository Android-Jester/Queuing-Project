use crate::models::User;
use crate::schema::users;
use diesel::prelude::*;
use dotenvy::dotenv;
use schema::users::account_number;
use std::env;
pub mod models;
pub mod schema;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Unable to connect to {database_url}"))
}

pub fn signup_user(
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

pub fn login_user(
    conn: &mut MysqlConnection,
    acc_num: &String,
    pass_word: &String,
) -> Option<User> {
    // let db_data = conn.transaction(|conn| {});
    let user = users::table
        .filter(account_number.eq(acc_num))
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
