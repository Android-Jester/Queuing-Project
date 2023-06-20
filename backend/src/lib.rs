use data::models::UserQueuePos;

pub mod data;
pub mod data_source;
pub mod interface;
#[cfg(test)]
pub mod tests;

pub struct Servers {
    pub server_1: Vec<UserQueuePos>,
    pub server_2: Vec<UserQueuePos>,
    pub server_3: Vec<UserQueuePos>,
    pub server_4: Vec<UserQueuePos>,
}

impl Servers {
    pub fn new() -> Self {
        Self {
            server_1: vec![],
            server_2: vec![],
            server_3: vec![],
            server_4: vec![],
        }
    }
    pub fn add_server_customer(&mut self, server_index: usize, user: UserQueuePos) -> Result<&mut self, &str> {
        match server_index {
            1 => {
                if self.server_1.len() < usize::MAX {
                    self.server_1.push(user);
                    Ok(self)
                } else {
                    Err("Unable to add customer")
                }
            },
            2 => {
                if self.server_2.len() < usize::MAX {
                    self.server_2.push(user);
                    Ok(self)
                } else {
                    Err("Unable to add customer")
                }
            },
            3 => {
                if self.server_3.len() < usize::MAX {
                    self.server_3.push(user);
                    Ok(self)
                } else {
                    Err("Unable to add customer")
                }
            },
            0 => {
                if self.server_4.len() < usize::MAX {
                    self.server_4.push(user);
                    Ok(self)
                } else {
                    Err("Unable to add customer")
                }
            },
            _ => {
                Err("Server Does not exist")
            }
        }
    }



    pub fn remove_server_customer(&mut self, user: UserQueuePos, server_index: usize,) -> Result<&mut self, &str> {
        if let Some(index) = user.teller_queue_pos {
            match server_index {
                1 => {
                    if self.server_1.len() > 0 {
                        self.server_1.remove(index);
                        Ok(self)
                    } else {
                        Err("Unable to add customer")
                    }
                },
                2 => {
                    if self.server_2.len() > 0 {
                        self.server_2.remove(index);
                        Ok(self)
                    } else {
                        Err("Unable to add customer")
                    }
                },
                3 => {
                    if self.server_3.len() > 0 {
                        self.server_3.remove(index);
                        Ok(self)
                    } else {
                        Err("Unable to add customer")
                    }
                },
                0 => {
                    if self.server_4.len() > 0 {
                        self.server_4.remove(index);
                        Ok(self)
                    } else {
                        Err("Unable to add customer")
                    }
                },
                _ => {
                    Err("Server Does not exist")
                }
            }
        }
        else {
            Err("User Does not exist")
        }


    }


}
