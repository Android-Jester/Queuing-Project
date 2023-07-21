use crate::prelude::*;
pub fn add_transaction(transaction_data: Transaction) -> Result<usize, diesel::result::Error> {
    let conn = &mut establish_connection();
    conn.transaction(|conn| {
        diesel::insert_into(Transactions::dsl::Transactions)
            .values(transaction_data)
            .execute(conn)
    })
}
