
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};


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


#[derive(Selectable, Queryable, Insertable)]
#[diesel(table_name = crate::data::schema::transaction)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Transaction {
    pub transaction_detail: Option<String>,
    pub user_account_number: Option<String>,
    pub server_id: Option<String>,
    pub duration: Option<f32>,
    pub transaction_time: Option<NaiveDateTime>,
}

// #[derive(Selectable, Queryable, Insertable)]
// #[diesel(table_name = crate::data::schema::users)]
// #[diesel(check_for_backend(diesel::mysql::Mysql))]
// pub struct User {
//     account_number: String
// }

impl Transaction {
    fn new(
        account_number: String,
        user_account_number: Option<String>,
        transaction: TransactionsType,
        server_identification: Option<String>,
        transaction_time: Option<NaiveDateTime>
        ) -> Transaction {
        let (action, duration) = select_transaction(transaction);
        Transaction {
            transaction_detail: Some(action),
            user_account_number,
            server_id: server_identification,
            duration: Some(duration),
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



