
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Deserializer, Serialize};

pub enum TransactionsType {
    Deposit {
        service_time: f32
    },
    Withdrawal{
        service_time: f32
    },
    ForeignExchange {
        service_time: f32
    },
    BillPayment {
        service_time: f32
    }
}



#[derive(Selectable, Queryable, Insertable, Deserialize)]
#[diesel(table_name = crate::data::schema::transaction)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Transaction {
    pub transaction_detail: String,
    pub server_id: String,
    pub user_account_number: String,
    pub duration: f32,
    pub transaction_time: NaiveDateTime,
}


#[derive(Selectable, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::data::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    account_number: String,
}

impl Transaction {
    fn new(
        account_number: String,
        user_account_number: String,
        transaction: TransactionsType,
        server_identification: String,
        transaction_time: NaiveDateTime
        ) -> Transaction {
        let (action, duration) = select_transaction(transaction);
        Transaction {
            transaction_detail: action,
            user_account_number,
            server_id: server_identification,
            duration,
            transaction_time
        }
    }
}

fn select_transaction(transaction: TransactionsType) -> (String, f32) {

    match transaction {
        TransactionsType::Deposit { service_time } => {
            (format!("deposit"), service_time)
        },
        TransactionsType::Withdrawal { service_time } => {
            (format!("withdrawal"), service_time)

        },
        TransactionsType::ForeignExchange { service_time } => {
            (format!("foreign_exchange"), service_time)
        },
        TransactionsType::BillPayment { service_time } => {
            (format!("payment"), service_time)
        },
    }
}



