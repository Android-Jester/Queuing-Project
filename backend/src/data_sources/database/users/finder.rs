use crate::prelude::*;
pub fn db_find_user(national_id: String) -> Result<ClientQuery, &'static str> {
    let conn = &mut establish_connection();
    match conn.transaction(|connection| {
        Clients::dsl::Clients
            .select(ClientQuery::as_select())
            .filter(Clients::national_id.eq(national_id))
            .first(connection)
    }) {
        Ok(client) => Ok(client),
        Err(_) => Err("Unable to Find User"),
    }
}

pub fn db_list_clients() -> Result<Vec<ClientQuery>, &'static str> {
    let conn = &mut establish_connection();
    match conn.transaction(|connection| {
        Clients::dsl::Clients
            .select(ClientQuery::as_select())
            .load(connection)
    }) {
        Ok(transactions) => Ok(transactions),
        Err(_) => Err("Unable to find transactions"),
    }
}
