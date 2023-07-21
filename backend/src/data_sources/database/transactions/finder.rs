use crate::prelude::*;

pub fn list_transactions() -> Result<Vec<Transaction>, &'static str> {
    let conn = &mut establish_connection();
    match conn.transaction(|connection| {
        Transactions::dsl::Transactions
            .select(Transaction::as_select())
            .load(connection)
    }) {
        Ok(transactions) => Ok(transactions),
        Err(_) => Err("Unable to find transactions"),
    }
}
