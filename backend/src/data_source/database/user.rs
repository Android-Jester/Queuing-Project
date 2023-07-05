use super::prelude::*;
use crate::data::prelude::*;
use diesel::prelude::*;

// pub fn register_user(user_insert_data: UserInsert) -> Result<usize, diesel::result::Error> {
//     let conn = &mut establish_connection();
//     conn.transaction(|conn| {
//         diesel::insert_into(Users::dsl::Users)
//             .values(user_insert_data)
//             .execute(conn)
//     })
// }

pub fn db_list_users() -> Result<Vec<UserInsert>, &'static str> {
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

/*Authentication */
pub fn db_check_user(login_data: UserLogin) -> Result<UserDataQuery, &'static str> {
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
                Ok(UserDataQuery::new(
                    user.name,
                    user.account_number,
                    user.national_id,
                ))
            } else {
                Err("Unable to login User")
            }
        }
        Err(_) => Err("Unable to Find User"),
    }
}
pub fn db_add_guest(insert_data: GuestInsert) -> Result<GuestInsert, &'static str> {
    let conn = &mut establish_connection();
    let data = conn.transaction(|conn| {
        diesel::insert_into(Guests::dsl::Guests)
            .values(insert_data.clone())
            .execute(conn)
    });
    match data {
        Ok(_) => Ok(insert_data),
        Err(_) => Err("Unable to add guests"),
    }
}

pub fn db_find_user(national_id: String) -> Result<UserQuery, &'static str> {
    let conn = &mut establish_connection();
    let transactions_data = conn.transaction(|connection| {
        Users::dsl::Users
            .select(UserQuery::as_select())
            .filter(Users::national_id.eq(national_id))
            .first(connection)
    });
    match transactions_data {
        Ok(teller) => Ok(teller),
        Err(_) => Err("Unable to Find User"),
    }
}
