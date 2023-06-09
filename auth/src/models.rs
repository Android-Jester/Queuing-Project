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
#[diesel(table_name = crate::schema::guest)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Guest {}
