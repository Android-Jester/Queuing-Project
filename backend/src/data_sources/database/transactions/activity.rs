use crate::prelude::*;
use diesel::dsl::now;
use diesel::RunQueryDsl;
pub fn add_transaction(transaction_data: Transaction) -> Result<usize, diesel::result::Error> {
    let conn = &mut establish_connection();
    conn.transaction(|conn| {
        diesel::insert_into(Transactions::dsl::Transactions)
            .values(transaction_data)
            .execute(conn)
    })
}

pub fn get_daily_report() -> Result<Vec<Transaction>, diesel::result::Error> {
    let conn = &mut establish_connection();
    conn.transaction(|conn| {
        Transactions::dsl::Transactions
            .select(Transaction::as_select())
            .order(Transactions::created_date.asc())
            .filter(Transactions::created_date.lt(now))
            .load(conn)
    })
}

pub fn get_weekly_report() -> Result<Vec<Transaction>, diesel::result::Error> {
    let conn = &mut establish_connection();
    diesel::sql_query("SELECT * FROM Transactions WHERE created_date > NOW() - INTERVAL 1 WEEK")
        .get_results(conn)
}

pub fn get_monthly_report() -> Result<Vec<Transaction>, diesel::result::Error> {
    let conn = &mut establish_connection();
    let transaction = diesel::sql_query(
        "SELECT * FROM Transactions WHERE created_date > NOW() - INTERVAL 1 MONTH",
    )
    .get_results(conn);
    transaction
}

pub fn get_year_report() -> Result<Vec<Transaction>, diesel::result::Error> {
    let conn = &mut establish_connection();
    diesel::sql_query("SELECT * FROM Transactions WHERE created_date > NOW() - INTERVAL 1 YEAR")
        .load(conn)
}
