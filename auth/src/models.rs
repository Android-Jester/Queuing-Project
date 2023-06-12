use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub account_number: String,
    pub password: String,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::guests)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Guest {
    pub name: Option<String>,
    pub action: Option<String>,
    pub national_id: String
}