use serde::{Deserialize, Serialize};

use crate::data::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct JoinedUserOutput {
    pub user_query: UserQuery,
    pub action: String,
}


impl JoinedUserOutput {
   pub fn new(user_query: UserQuery, action: String) -> Self {
        Self {
            user_query,
            action
        }
    }
}

