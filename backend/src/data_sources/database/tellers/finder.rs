use crate::prelude::*;
pub fn db_list_tellers() -> Result<Vec<ServerQuery>, &'static str> {
    let conn = &mut establish_connection();
    match conn.transaction(|connection| {
        Servers::dsl::Servers
            .select(ServerQuery::as_select())
            .load(connection)
    }) {
        Ok(transactions) => Ok(transactions),
        Err(_) => Err("Unable to find transactions"),
    }
}

// TODO: Fix this function to return model that doesn't contain password
pub fn find_teller(teller_id: String) -> Result<ServerQuery, &'static str> {
    let conn = &mut establish_connection();
    let transactions_data = conn.transaction(|connection| {
        Servers::dsl::Servers
            .select(ServerQuery::as_select())
            .find(teller_id)
            .first(connection)
    });
    match transactions_data {
        Ok(teller) => {
            info!("Teller: {:?}", teller);
            Ok(teller) },
        Err(_) => {
            error!("Unable to find teller");
            Err("Unable to Find Telle")
        },
    }
}
