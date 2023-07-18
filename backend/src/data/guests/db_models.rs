use crate::prelude::*;

#[derive(Selectable, Queryable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = Guests)]
#[diesel(check_for_backend(Mysql))]
pub struct GuestQuery {
    pub national_id: String,
    pub name: String,
    pub transaction_type: String,
    pub telephone_num: String,
}
