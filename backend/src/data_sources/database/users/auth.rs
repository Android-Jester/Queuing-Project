use crate::prelude::*;
// TODO: Use only one query object
pub fn db_login_user(login_data: UserLogin) -> Result<UserDataQuery, &'static str> {
    let conn = &mut establish_connection();
    match conn.transaction(|connection| {
        Users::dsl::Users
            .select(UserQuery::as_select())
            .filter(Users::account_number.eq(login_data.account_number))
            .first(connection)
    }) {
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

pub fn db_find_user(national_id: String) -> Result<UserQuery, &'static str> {
    let conn = &mut establish_connection();
    match conn.transaction(|connection| {
        Users::dsl::Users
            .select(UserQuery::as_select())
            .filter(Users::national_id.eq(national_id))
            .first(connection)
    }) {
        Ok(teller) => Ok(teller),
        Err(_) => Err("Unable to Find User"),
    }
}

pub fn db_list_users() -> Result<Vec<UserQuery>, &'static str> {
    let conn = &mut establish_connection();
    match conn.transaction(|connection| {
        Users::dsl::Users
            .select(UserQuery::as_select())
            .load(connection)
    }) {
        Ok(transactions) => Ok(transactions),
        Err(_) => Err("Unable to find transactions"),
    }
}
