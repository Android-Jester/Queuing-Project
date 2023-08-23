use crate::prelude::*;
use chrono::NaiveDateTime;

#[derive(
    Selectable, Queryable, Insertable, Deserialize, QueryableByName, Serialize, Clone, Debug,
)]
#[diesel(table_name = Transactions)]
#[diesel(check_for_backend(Mysql))]
pub struct Transaction {
    pub detail: String,
    pub server_id: String,
    pub client_national_id: String,
    pub duration: f32,
    #[serde(skip_serializing, skip_deserializing)]
    pub created_date: NaiveDateTime,
}

impl Transaction {
    pub fn new(
        detail: String,
        client_national_id: String,
        server_id: String,
        duration: std::time::Duration,
        created_date: NaiveDateTime,
    ) -> Transaction {
        Transaction {
            detail,
            client_national_id,
            server_id,
            duration: duration.as_secs_f32() / 60.0,
            created_date,
        }
    }
}
