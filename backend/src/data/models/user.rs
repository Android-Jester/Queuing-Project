use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Selectable, Queryable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = crate::data::schema::Users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserQuery {
    pub name: String,
    pub account_number: String,
    pub national_id: String,
}

// #[derive(Selectable, Queryable, Serialize, Deserialize, Clone, Debug)]
// #[diesel(table_name = crate::data::schema::Users)]
// #[diesel(check_for_backend(diesel::mysql::Mysql))]
// pub struct UserServerQuery {

//     pub national_id: String,
// }

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserQueuePos {
    pub national_id: String,
    pub action: String,
    pub queue_pos: usize,
    pub teller_queue_pos: Option<usize>,
    pub assigned_teller: Option<usize>,
    pub timer: f64,
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
