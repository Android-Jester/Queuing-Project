use super::prelude::*;
use crate::data::prelude::*;
use diesel::prelude::*;
use diesel::result::Error;

pub fn db_check_teller(teller_login: TellerLogin) -> Result<(String, i32, f64), &'static str> {
    let conn = &mut establish_connection();
    let transactions_data = conn.transaction(|connection| {
        Tellers::dsl::Tellers
            .select(TellerQuery::as_select())
            .find(teller_login.server_id)
            .first(connection)
    });
    match transactions_data {
        Ok(teller) => {
            if teller.password.eq(&teller_login.password) {
                Ok((teller.server_id, teller.server_station, teller.service_time as f64))
            } else {
                Err("Unable to login Teller")
            }
        }
        Err(_) => Err("Unable to Find Teller"),
    }
}

pub fn find_teller(teller_id: String) -> Result<Teller, &'static str> {
    let conn = &mut establish_connection();
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
    let conn = &mut establish_connection();
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

/* Teller Actions */
pub fn set_teller_status(status: bool, teller_id: String) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    conn.transaction(|connection| {
        diesel::update(Tellers::dsl::Tellers.find(teller_id))
            .set(Tellers::active.eq(status))
            .execute(connection)
    })
}
