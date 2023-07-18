use crate::prelude::*;
pub fn add_transaction(transaction_data: Transaction) -> Result<usize, diesel::result::Error> {
    let conn = &mut establish_connection();
    conn.transaction(|conn| {
        diesel::insert_into(Transactions::dsl::Transactions)
            .values(transaction_data)
            .execute(conn)
    })
}

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
