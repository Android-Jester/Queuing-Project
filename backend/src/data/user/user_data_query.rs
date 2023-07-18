use crate::prelude::*;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserDataQuery {
    pub name: String,
    pub account_number: String,
    pub national_id: String,
}

impl UserDataQuery {
    pub fn new(name: String, account_number: String, national_id: String) -> Self {
        Self {
            name,
            account_number,
            national_id,
        }
    }
}
