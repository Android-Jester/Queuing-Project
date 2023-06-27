use super::prelude::*;
use crate::data::prelude::*;
use diesel::prelude::*;
use diesel::result::Error;

// pub fn register_user(user_insert_data: UserInsert) -> Result<usize, diesel::result::Error> {
//     let conn = &mut establish_connection();
//     conn.transaction(|conn| {
//         diesel::insert_into(Users::dsl::Users)
//             .values(user_insert_data)
//             .execute(conn)
//     })
// }

pub fn list_users_db() -> Result<Vec<UserInsert>, &'static str> {
    let conn = &mut establish_connection();
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

pub fn register_guest(insert_data: GuestInsert) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    conn.transaction(|conn| {
        diesel::insert_into(Guests::dsl::Guests)
            .values(insert_data)
            .execute(conn)
    })
}

/*Authentication */
pub fn login_user(login_data: UserLogin) -> Result<UserLoginQuery, &'static str> {
    let conn = &mut establish_connection();
    let user_data = conn.transaction(|connection| {
        Users::dsl::Users
            .select(UserLoginQuery::as_select())
            .filter(Users::account_number.eq(login_data.account_number))
            .first(connection)
    });
    match user_data {
        Ok(user) => {
            if user.password.eq(&login_data.password) {
                Ok(user)
            } else {
                Err("Unable to login User")
            }
        }
        Err(_) => Err("Unable to Find User"),
    }
}
pub fn login_guest(guest: Guest) -> Result<String, &'static str> {
    let conn = &mut establish_connection();
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

pub fn find_user(user_national_id: String) -> Result<UserQuery, &'static str> {
    let conn = &mut establish_connection();
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
