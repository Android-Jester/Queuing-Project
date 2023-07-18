use crate::prelude::*;

// TODO: Change Models to be returnable
pub fn db_auth_teller(teller_login: TellerLogin) -> Result<(String, i32, f64), &'static str> {
    let conn = &mut establish_connection();
    match conn.transaction(|connection| {
        Tellers::dsl::Tellers
            .select(TellerQuery::as_select())
            .find(teller_login.server_id)
            .first(connection)
    }) {
        Ok(teller) => {
            if teller.password.eq(&teller_login.password) {
                // TODO: Convert Data to Struct
                Ok((
                    teller.server_id,
                    teller.server_station,
                    teller.service_time as f64,
                ))
            } else {
                Err("Unable to login Teller")
            }
        }
        Err(_) => Err("Unable to Find Teller"),
    }
}

pub fn db_list_tellers() -> Result<Vec<TellerQuery>, &'static str> {
    let conn = &mut establish_connection();
    match conn.transaction(|connection| {
        Tellers::dsl::Tellers
            .select(TellerQuery::as_select())
            .load(connection)
    }) {
        Ok(transactions) => Ok(transactions),
        Err(_) => Err("Unable to find transactions"),
    }
}
