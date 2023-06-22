use std::fmt::Display;

use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

pub enum TransactionsType {
    Deposit { service_time: f32 },
    Withdrawal { service_time: f32 },
    ForeignExchange { service_time: f32 },
    BillPayment { service_time: f32 },
}

#[derive(Selectable, Queryable, Insertable, Deserialize)]
#[diesel(table_name = crate::data::schema::Transactions)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Transaction {
    pub transaction_detail: String,
    pub server_id: String,
    pub national_id: Option<String>,
    pub guest_national_id: Option<String>,
    pub duration: f32,
    pub transaction_time: NaiveDateTime,
}

#[derive(Selectable, Queryable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = crate::data::schema::Users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserQuery {
    pub national_id: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UserQueuePos {
    pub national_id: String,
    pub queue_pos: usize,
    pub teller_queue_pos: Option<usize>,
    pub assigned_teller: Option<usize>,
}

#[derive(Deserialize, Clone)]
pub struct UserQueryData {
    pub national_id: String,
    pub action: String,
}

#[derive(Queryable, Insertable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::data::schema::Users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserInsert {
    pub account_number: String,
    pub national_id: String,
    pub password: String,
}

#[derive(Selectable, Queryable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::data::schema::Users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserLoginQuery {
    pub account_number: String,
    pub password: String,
    pub national_id: String,
}

#[derive(Selectable, Queryable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::data::schema::Users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserLogin {
    pub account_number: String,
    pub password: String,
}

#[derive(Selectable, Queryable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::data::schema::Guests)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Guest {
    pub national_id: String,
    pub name: String,
    pub transaction_type: String,
    pub telephone_num: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::data::schema::Guests)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct GuestInsert {
    pub national_id: String,
    pub name: String,
    pub transaction_type: String,
    pub telephone_num: String,
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
            guest_national_id,
            server_id: server_identification,
            duration,
            transaction_time,
        }
    }
}

#[derive(Selectable, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::data::schema::Tellers)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Teller {
    pub server_id: String,
    pub server_station: i32,
    pub service_time: f32,
    pub active: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::data::schema::Tellers)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct TellerInsert {
    pub server_id: String,
    pub server_station: i32,
    pub service_time: f32,
    pub active: bool,
    pub password: String,
}

#[derive(Selectable, Queryable, Deserialize, Serialize)]
#[diesel(table_name = crate::data::schema::Tellers)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct TellerLogin {
    pub server_id: String,
    pub password: String,
}

impl Teller {
    pub fn change_teller_status(&mut self, status: bool) -> &mut Teller {
        self.active = status;
        self
    }
}

impl Display for Teller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}:{}:{}:{}",
            self.server_id, self.server_station, self.service_time, self.active,
        ))
    }
}
