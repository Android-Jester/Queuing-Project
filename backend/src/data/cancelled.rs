use crate::prelude::*;
#[derive(
    Selectable, Queryable, Insertable, Deserialize, QueryableByName, Serialize, Clone, Debug,
)]
#[diesel(table_name = Cancelled)]
#[diesel(check_for_backend(Mysql))]
pub struct CancelStruct {
    pub detail: String,
    pub server_id: String,
    pub client_national_id: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub created_date: chrono::NaiveDateTime,
}
