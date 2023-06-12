use std::env;
use crate::data::schema::transaction::dsl::*;
use crate::data::{models::Transaction, schema::transaction};
use diesel::prelude::*;
use diesel::{Connection, MysqlConnection};
use diesel::result::Error;
use dotenvy::dotenv;

pub fn establish_conn() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url).expect("Unable to connect to DB")
}

pub fn add_transaction(transaction_data: Transaction) -> Result<usize, Error> {
    let conn = &mut establish_conn();
    let insert_transaction = conn.transaction(|conn| {
        diesel::insert_into(transaction::table)
            .values(transaction_data)
            .execute(conn)
    });
    insert_transaction
   
}
pub fn get_all_service_times() -> Vec<(Vec<f64>, f64)> {
    let conn = &mut establish_conn();
    let mut data: Vec<f64> = Vec::new();
    let transactions_data = conn
        .transaction(|connection| {
            let results = transaction
                .select(Transaction::as_select())
                .load(connection);
            results
        })
        .expect("Unknown Values");
    vec![(vec![0.0], 0.0)]
}