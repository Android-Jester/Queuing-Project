use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

pub enum Transactions {
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


#[derive(Insertable, Selectable, Queryable)]
#[diesel(table_name = crate::data::schema::Users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    account_number: String,
    transaction: Option<String>,
    service_time: f32
}

impl User {
    fn new(account_number: String, transaction: Transactions) -> User {
        let (action, duration) = select_transaction(transaction);
        User {
            account_number,
            transaction: Some(action),
            service_time: duration
        }
    }
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::data::schema::teller)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Teller {
    pub teller_id: String,
    pub transaction: String,
    pub service_time: f32
}

fn select_transaction(transaction: Transactions) -> (String, f32) {

    match transaction {
        Transactions::Deposit { service_time } => {
            (format!("deposit"), service_time)
        },
        Transactions::Withdrawal { service_time } => {
            (format!("withdrawal"), service_time)

        },
        Transactions::ForeignExchange { service_time } => {
            (format!("foreign_exchange"), service_time)
        },
        Transactions::BillPayment { service_time } => {
            (format!("payment"), service_time)
        },
    }
}


impl Teller {
    fn new(id: String, action: Transactions) -> Teller {
        let (transaction, duration) = select_transaction(action);
        Teller {
            teller_id: id,
            transaction,
            service_time: duration.into()
        }
    }
}


