use crate::prelude::*;

#[derive(Selectable, Queryable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = crate::data::schema::Users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserQuery {
    pub name: String,
    pub account_number: String,
    pub password: String,
    pub national_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserLogin {
    pub account_number: String,
    pub password: String,
}
