use data::prelude::*;
use interface::{teller::teller_queue::*, user::models::*};
#[cfg(test)]
pub mod tests;
pub mod data;
pub mod data_source;
pub mod interface;

pub mod prelude {
    pub use super::data::prelude::*;
    pub use super::data_source::prelude::*;
    pub use super::interface::prelude::*;
}



#[derive(Default)]
pub struct Servers {
    pub server_1: Vec<JoinedUserOutput>,
    pub server_2: Vec<JoinedUserOutput>,
    pub server_3: Vec<JoinedUserOutput>,
    pub server_4: Vec<JoinedUserOutput>,
}

impl Servers {
    // pub fn new() -> Self {}
    pub fn add_server_customer(
        &mut self,
        server_index: usize,
        user: JoinedUserOutput,
    ) -> Result<(usize, usize), &str> {
        match server_index {
            1 => {
                if self.server_1.len() < usize::MAX {
                    self.server_1.push(user);
                    Ok((1, self.server_1.len()))
                } else {
                    Err("Unable to add customer")
                }
            }
            2 => {
                if self.server_2.len() < usize::MAX {
                    self.server_2.push(user);
                    Ok((2, self.server_2.len()))
                } else {
                    Err("Unable to add customer")
                }
            }
            3 => {
                if self.server_3.len() < usize::MAX {
                    self.server_3.push(user);
                    Ok((3, self.server_3.len()))
                } else {
                    Err("Unable to add customer")
                }
            }
            0 => {
                if self.server_4.len() < usize::MAX {
                    self.server_4.push(user);
                    Ok((4, self.server_4.len()))
                } else {
                    Err("Unable to add customer")
                }
            }
            _ => Err("Server Does not exist"),
        }
    }

    pub fn remove_server_customer(
        &mut self,
        user: UserQueuePos,
        server_index: usize,
    ) -> Result<&mut Self, &str> {
        if let Some(index) = user.teller_queue_pos {
            match server_index {
                1 => {
                    if self.server_1.is_empty() {
                        self.server_1.remove(index);
                        Ok(self)
                    } else {
                        Err("Unable to add customer")
                    }
                }
                2 => {
                    if self.server_2.is_empty() {
                        self.server_2.remove(index);
                        Ok(self)
                    } else {
                        Err("Unable to add customer")
                    }
                }
                3 => {
                    if self.server_3.is_empty() {
                        self.server_3.remove(index);
                        Ok(self)
                    } else {
                        Err("Unable to add customer")
                    }
                }
                0 => {
                    if self.server_4.is_empty() {
                        self.server_4.remove(index);
                        Ok(self)
                    } else {
                        Err("Unable to add customer")
                    }
                }
                _ => Err("Server Does not exist"),
            }
        } else {
            Err("User Does not exist")
        }
    }

    pub fn show_users(&self, teller_loc: usize) -> Vec<ServerTeller> {
        match teller_loc {
            1 => {
                let mut teller_view: Vec<ServerTeller> = Vec::new();
                for user in self.server_1.clone() {
                    let server_tel = ServerTeller {
                        national_id: user.user_query.national_id,
                        account_number: user.user_query.account_number,
                        action: user.action,
                        name: user.user_query.name,
                    };
                    teller_view.push(server_tel);
                }
                teller_view
            }
            2 => {
                let mut teller_view: Vec<ServerTeller> = Vec::new();
                for user in self.server_2.clone() {
                    let server_tel = ServerTeller {
                        national_id: user.user_query.national_id,
                        account_number: user.user_query.account_number,
                        action: user.action,
                        name: user.user_query.name,
                    };
                    teller_view.push(server_tel);
                }
                teller_view
            }
            3 => {
                let mut teller_view: Vec<ServerTeller> = Vec::new();
                for user in self.server_3.clone() {
                    let server_tel: ServerTeller = ServerTeller {
                        national_id: user.user_query.national_id,
                        account_number: user.user_query.account_number,
                        action: user.action,
                        name: user.user_query.name,
                    };
                    teller_view.push(server_tel);
                }
                teller_view
            }
            0 => {
                let mut teller_view: Vec<ServerTeller> = Vec::new();
                for user in self.server_4.clone() {
                    let server_tel = ServerTeller {
                        national_id: user.user_query.national_id,
                        account_number: user.user_query.account_number,
                        action: user.action,
                        name: user.user_query.name,
                    };
                    teller_view.push(server_tel);
                }
                teller_view
            }
            _ => {
                let mut teller_view: Vec<ServerTeller> = Vec::new();
                for user in self.server_1.clone() {
                    let server_tel = ServerTeller {
                        national_id: user.user_query.national_id,
                        account_number: user.user_query.account_number,
                        action: user.action,
                        name: user.user_query.name,
                    };
                    teller_view.push(server_tel);
                }
                teller_view
            }
        }
    }
}
