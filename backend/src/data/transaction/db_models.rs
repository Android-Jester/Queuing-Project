use crate::prelude::*;
use chrono::NaiveDateTime;

#[derive(Selectable, Queryable, Insertable, Deserialize, Serialize, Clone)]
#[diesel(table_name = Transactions)]
#[diesel(check_for_backend(Mysql))]
pub struct Transaction {
    pub transaction_detail: String,
    pub server_id: String,
    pub national_id: Option<String>,
    pub duration: f32,
    pub transaction_time: NaiveDateTime,
}

impl Transaction {
    pub fn new(
        transaction_detail: String,
        national_id: Option<String>,
        server_id: String,
        duration: std::time::Duration,
        transaction_time: NaiveDateTime,
    ) -> Transaction {
        Transaction {
            transaction_detail,
            national_id,
            server_id,
            duration: duration.as_secs_f32() / 60.0,
            transaction_time,
        }
    }
}
