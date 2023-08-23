use crate::prelude::*;

#[derive(Selectable, Queryable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = Clients)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct ClientQuery {
    pub name: String,
    pub account_number: String,
    pub password: String,
    pub national_id: String,
}
