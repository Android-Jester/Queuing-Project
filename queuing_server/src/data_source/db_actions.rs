use std::env;
use crate::data::schema::transaction::dsl::*;
use crate::data::{models::Transaction, schema::transaction};
use diesel::prelude::*;
use diesel::{Connection, MysqlConnection};
use dotenvy::dotenv;

pub fn establish_conn() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url).expect("Unable to connect to DB")
}

fn add_transaction(conn: &mut MysqlConnection, customer: Transaction) -> bool {
    let insert_transaction = conn.transaction(|conn| {
        diesel::insert_into(transaction::table)
            .values(customer)
            .execute(conn)
    });
    match insert_transaction {
        Ok(_) => true,
        Err(_) => false,
    }
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