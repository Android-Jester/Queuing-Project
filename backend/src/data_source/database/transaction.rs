use super::prelude::*;
use crate::data::prelude::*;
use diesel::prelude::*;
use diesel::result::Error;

/*Adding Data*/
pub fn add_transaction(transaction_data: Transaction) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    conn.transaction(|conn| {
        diesel::insert_into(Transactions::dsl::Transactions)
            .values(transaction_data)
            .execute(conn)
    })
}

/// Obtain all service Times and sort them according to the randomforest model
pub fn list_transactions() -> Result<Vec<Transaction>, &'static str> {
    let conn = &mut establish_connection();
    let transactions_data = conn.transaction(|connection| {
        Transactions::dsl::Transactions
            .select(Transaction::as_select())
            .load(connection)
    });

    match transactions_data {
        Ok(transactions) => Ok(transactions),
        Err(_) => Err("Unable to find transactions"),
    }
}
