use crate::prelude::*;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct TellerQueueStruct {
    pub teller_position: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ServerTeller {
    pub name: String,
    pub account_number: String,
    pub action: String,
    pub national_id: String,
}
#[derive(Default, Debug)]
pub struct TellersQueue {
    tellers: Vec<Option<ServerQueue>>,
}

impl TellersQueue {
    pub fn tellers_num(&self) -> usize {
        self.tellers.len()
    }
    fn reassign_tellers(&mut self) {
        for teller in &mut self.tellers {
            match teller {
                Some(queue) => queue.users = vec![],
                None => {}
            }
        }
    }
    pub fn add_teller(&mut self, teller: TellerQueueQuery) -> Result<(), &str> {
        match self.tellers_num() < SERVER_COUNT {
            true => {
                self.tellers.push(Some(ServerQueue::new(teller)));
                info!("{:?}", self.tellers);
                Ok(())
            }
            false => Err("Server List full"),
        }
    }
    pub fn remove_teller(&mut self, index: usize) -> Result<Option<ServerQueue>, &str> {
        if index > 0 || index > self.tellers_num() {
            Ok(self.tellers.remove(index))
        } else {
            Err("Index not in line")
        }
    }
    pub fn search_teller(&self, station: usize) -> Result<&ServerQueue, &str> {
        if station < self.tellers.len() {
            match self.tellers[station].as_ref() {
                Some(teller) => Ok(teller),
                None => Err("Unable to find teller"),
            }
        } else {
            Err("No Available Teller")
        }
    }
    pub fn add_customer(
        &mut self,
        server_station: usize,
        user: JoinedUserOutput,
    ) -> Result<(usize, usize), &str> {
        info!("Server Station: {}", server_station);
        info!("Teller 1: {:?}", self.tellers[0]);
        match &mut self.tellers[server_station] {
            Some(teller) => match teller.users.len() != usize::MAX {
                true => {
                    teller.users.push(user);
                    // Teller positon, User Position
                    Ok((server_station, teller.users.len()))
                }
                false => Err("Unable to add customer"),
            },
            None => Err("No Teller Available"),
        }
    }
    pub fn remove_customer(&mut self, user: UserQueuePos) -> Result<(), &str> {
        match user.server_pos {
            Some(index) => match &mut self.tellers[index] {
                Some(teller) => match teller.users.is_empty() {
                    true => {
                        {
                            teller.users.remove(index);
                            self.reassign_tellers();
                        };
                        Ok(())
                    }
                    false => Err("Unable to add customer"),
                },
                None => Err("No User Found"),
            },
            None => Err("No Teller Found"),
        }
    }
    pub fn show_users(&mut self, service_location: usize) -> Vec<ServerTeller> {
        match &mut self.tellers[service_location] {
            Some(teller) => {
                let mut teller_view: Vec<ServerTeller> = Vec::new();
                for user in teller.users.clone() {
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
            None => vec![],
        }
    }
}
// #[derive(Default)]
// pub struct Servers {
//     pub server_1: Vec<JoinedUserOutput>,
//     pub server_2: Vec<JoinedUserOutput>,
//     pub server_3: Vec<JoinedUserOutput>,
//     pub server_4: Vec<JoinedUserOutput>,
// }

// impl Servers {
//     fn add_item_server(
//         list: &mut Vec<JoinedUserOutput>,
//         item: JoinedUserOutput,
//     ) -> Result<(usize, usize), &'static str> {
//         if list.len() == usize::MAX {
//             return Err("Unable to add customer");
//         }
//         list.push(item);
//         Ok((1, list.len()))
//     }
//     pub fn add_server_customer(
//         &mut self,
//         server_index: usize,
//         user: JoinedUserOutput,
//     ) -> Result<(usize, usize), &str> {
//         match server_index {
//             1 => Self::add_item_server(&mut self.server_1, user),
//             2 => Self::add_item_server(&mut self.server_2, user),
//             3 => Self::add_item_server(&mut self.server_3, user),
//             0 => Self::add_item_server(&mut self.server_4, user),
//             _ => Err("Server Does not exist"),
// 2 => {
//     if self.server_2.len() < usize::MAX {
//         self.server_2.push(user);
//         Ok((2, self.server_2.len()))
//     } else {
//         Err("Unable to add customer")
//     }
// }
// 3 => {
//     if self.server_3.len() < usize::MAX {
//         self.server_3.push(user);
//         Ok((3, self.server_3.len()))
//     } else {
//         Err("Unable to add customer")
//     }
// }
// 0 => {
//     if self.server_4.len() < usize::MAX {
//         self.server_4.push(user);
//         Ok((4, self.server_4.len()))
//     } else {
//         Err("Unable to add customer")
//     }
// }
//         }
//     }

//     fn remove_item_servers(
//         list: &mut Vec<JoinedUserOutput>,
//         index: usize,
//     ) -> Result<(), &'static str> {
//         if !list.is_empty() {
//             return Err("Unable to add customer");
//         }
//         list.remove(index);
//         Ok(())
//     }

//     pub fn remove_server_customer(
//         &mut self,
//         user: UserQueuePos,
//         server_index: usize,
//     ) -> Result<(), &str> {
//         if let Some(index) = user.teller_queue_pos {
//             match server_index {
//                 1 => Self::remove_item_servers(&mut self.server_1, index),
//                 2 => Self::remove_item_servers(&mut self.server_2, index),
//                 3 => Self::remove_item_servers(&mut self.server_3, index),
//                 0 => Self::remove_item_servers(&mut self.server_4, index),
//                 _ => Err("Server Does not exist"),
//             }
//         } else {
//             Err("User Does not exist")
//         }
//     }

// fn show_item(&self, list: &Vec<JoinedUserOutput>, teller_loc: usize) -> Vec<ServerTeller> {
//     let mut teller_view: Vec<ServerTeller> = Vec::new();
//     for user in list.clone() {
//         let server_tel = ServerTeller {
//             national_id: user.user_query.national_id,
//             account_number: user.user_query.account_number,
//             action: user.action,
//             name: user.user_query.name,
//         };
//         teller_view.push(server_tel);
//     }
//     teller_view
// }

// pub fn show_users(&self, teller_loc: usize) -> Vec<ServerTeller> {
//     match teller_loc {
//         1 => self.show_item(&self.server_1, teller_loc),
//         2 => self.show_item(&self.server_2, teller_loc),
//         3 => self.show_item(&self.server_3, teller_loc),
//         0 => self.show_item(&self.server_4, teller_loc),
//         _ => self.show_item(&self.server_1, teller_loc),
// _ => {
//     let mut teller_view: Vec<ServerTeller> = Vec::new();
//     for user in self.server_1.clone() {
//         let server_tel = ServerTeller {
//             national_id: user.user_query.national_id,
//             account_number: user.user_query.account_number,
//             action: user.action,
//             name: user.user_query.name,
//         };
//         teller_view.push(server_tel);
//     }
//     teller_view
// } // 2 => {
//     let mut teller_view: Vec<ServerTeller> = Vec::new();
//     for user in self.server_2.clone() {
//         let server_tel = ServerTeller {
//             national_id: user.user_query.national_id,
//             account_number: user.user_query.account_number,
//             action: user.action,
//             name: user.user_query.name,
//         };
//         teller_view.push(server_tel);
//     }
//     teller_view
// }
// 3 => {
//     let mut teller_view: Vec<ServerTeller> = Vec::new();
//     for user in self.server_3.clone() {
//         let server_tel: ServerTeller = ServerTeller {
//             national_id: user.user_query.national_id,
//             account_number: user.user_query.account_number,
//             action: user.action,
//             name: user.user_query.name,
//         };
//         teller_view.push(server_tel);
//     }
//     teller_view
// }
// 0 => {
//     let mut teller_view: Vec<ServerTeller> = Vec::new();
//     for user in self.server_4.clone() {
//         let server_tel = ServerTeller {
//             national_id: user.user_query.national_id,
//             account_number: user.user_query.account_number,
//             action: user.action,
//             name: user.user_query.name,
//         };
//         teller_view.push(server_tel);
//     }
//     teller_view
// }
//     }
// }
// }
