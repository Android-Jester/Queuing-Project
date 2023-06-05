use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub account_number: String,
    pub password: String,
}
