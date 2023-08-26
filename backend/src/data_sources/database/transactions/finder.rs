use crate::prelude::*;
pub fn list_transactions() -> Result<Vec<Transaction>, &'static str> {
    let conn = &mut establish_connection();
    match conn.transaction(|connection| {
        Transactions::dsl::Transactions
            .select(Transaction::as_select())
            .load(connection)
    }) {
        Ok(transactions) => {
            info!("Transactions: {:?}", transactions);
            Ok(transactions)
        }
        Err(_) => Err("Unable to find transactions"),
    }
}

#[derive(QueryableByName, Queryable, Debug)]
#[diesel(table_name = Transactions)]
#[diesel(check_for_backend(Mysql))]
struct ServiceTimes {
    pub duration: f32,
}

pub fn get_service_time() -> Result<f32, String> {
    let conn = &mut establish_connection();
    let db_res = conn.transaction(|connection| {
        diesel::sql_query(
            "SELECT duration FROM Transactions WHERE created_date > NOW() - INTERVAL 1 DAY",
        )
        .get_results::<ServiceTimes>(connection)
    });

    match db_res {
        Ok(transactions) => {
            info!("Transactions: {:?}", transactions);
            let length = transactions.len();
            let mut sum: f32 = 0.0;
            for duration in transactions {
                sum += duration.duration;
            }
            let res = sum / length as f32;
            Ok(res)
        }
        Err(_) => Err("Unable to find transactions".to_string()),
    }
}
