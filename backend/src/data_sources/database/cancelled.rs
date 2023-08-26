use crate::prelude::*;
pub fn add_cancelled(transaction_data: CancelStruct) -> Result<usize, diesel::result::Error> {
    let conn = &mut establish_connection();
    conn.transaction(|conn| {
        diesel::insert_into(Cancelled::dsl::Cancelled)
            .values(transaction_data)
            .execute(conn)
    })
}

pub fn get_daily_cancelled_report() -> Result<Vec<CancelStruct>, diesel::result::Error> {
    let conn = &mut establish_connection();
    conn.transaction(|conn| {
        Cancelled::dsl::Cancelled
            .select(CancelStruct::as_select())
            .order(Cancelled::created_date.asc())
            .filter(Cancelled::created_date.lt(diesel::dsl::now))
            .load(conn)
    })
}

pub fn get_weekly_cancelled_report() -> Result<Vec<CancelStruct>, diesel::result::Error> {
    let conn = &mut establish_connection();
    diesel::sql_query("SELECT * FROM Cancelled WHERE created_date > NOW() - INTERVAL 1 WEEK")
        .load(conn)
}

pub fn get_monthly_cancelled_report() -> Result<Vec<CancelStruct>, diesel::result::Error> {
    let conn = &mut establish_connection();
    let transaction =
        diesel::sql_query("SELECT * FROM Cancelled WHERE created_date > NOW() - INTERVAL 1 MONTH")
            .get_results(conn);
    transaction
}

pub fn get_year_cancelled_report() -> Result<Vec<CancelStruct>, diesel::result::Error> {
    let conn = &mut establish_connection();
    diesel::sql_query("SELECT * FROM Cancelled WHERE created_date > NOW() - INTERVAL 1 YEAR")
        .load(conn)
}
