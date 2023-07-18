use crate::prelude::*;

// TODO: Fix this function to return model that doesn't contain password
pub fn find_teller(teller_id: String) -> Result<TellerQuery, &'static str> {
    let conn = &mut establish_connection();
    let transactions_data = conn.transaction(|connection| {
        Tellers::dsl::Tellers
            .select(TellerQuery::as_select())
            .find(teller_id)
            .first(connection)
    });
    match transactions_data {
        Ok(teller) => Ok(teller),
        Err(_) => Err("Unable to Find Telle"),
    }
}

pub fn set_teller_status(status: bool, teller_id: String) -> Result<usize, diesel::result::Error> {
    let conn = &mut establish_connection();
    conn.transaction(|connection| {
        diesel::update(Tellers::dsl::Tellers.find(teller_id))
            .set(Tellers::active.eq(status))
            .execute(connection)
    })
}
