use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub enum TransactionsType {
    Deposit { service_time: f32 },
    Withdrawal { service_time: f32 },
    ForeignExchange { service_time: f32 },
    BillPayment { service_time: f32 },
}

#[derive(Selectable, Queryable, Insertable, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::data::schema::Transactions)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Transaction {
    pub transaction_detail: String,
    pub server_id: String,
    pub national_id: Option<String>,
    // pub guest_national_id: Option<String>,
    pub duration: f32,
    pub transaction_time: NaiveDateTime,
}

impl Transaction {
    pub fn new(
        national_id: Option<String>,
        guest_national_id: Option<String>,
        transaction: TransactionsType,
        server_identification: String,
        transaction_time: NaiveDateTime,
    ) -> Transaction {
        fn select_transaction_type(transaction: TransactionsType) -> (String, f32) {
            match transaction {
                TransactionsType::Deposit { service_time } => ("deposit".to_string(), service_time),
                TransactionsType::Withdrawal { service_time } => {
                    ("withdrawal".to_string(), service_time)
                }
                TransactionsType::ForeignExchange { service_time } => {
                    ("foreign_exchange".to_string(), service_time)
                }
                TransactionsType::BillPayment { service_time } => {
                    ("payment".to_string(), service_time)
                }
            }
        }
        let (action, duration) = select_transaction_type(transaction);
        Transaction {
            transaction_detail: action,
            national_id,
            // guest_national_id,
            server_id: server_identification,
            duration,
            transaction_time,
        }
    }
}
