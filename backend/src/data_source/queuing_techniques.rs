use std::sync::MutexGuard;

use crate::data::models::{UserQuery, UserQueuePos};
use crate::data::{CUSTOMER_COUNT, SERVER_COUNT};
use crate::data_source::db_actions::{find_teller, list_transactions};
use crate::interface::teller_interface::TellersQueue;
use crate::interface::user_interface::show_user_waiting_time;
use crate::Servers;
// use std::f64::consts::E;
// ///Calculate best of service_times
pub fn calc_best_avg(avg_times: [f64; SERVER_COUNT]) -> u8 {
    let mut best = avg_times[0];
    for i in avg_times.iter() {
        best = best.max(*i)
    }
    let pos = avg_times.iter().position(|data| *data == best).unwrap();
    pos as u8
}

// fn calc_avg_time(service_times: Vec<f64>) -> f64 {
//     let mut sum = 0.0;
//     for time in &service_times {
//         sum += time;
//     }
//     sum / service_times.len() as f64
// }
//
// fn probability_of_none(arrival_rate: f64, time_interval: f64) -> f64 {
//     E.powf(-(arrival_rate * time_interval))
// }
//
// fn factorial(num: u8) -> u64 {
//     let mut mul: u64 = 1;
//     for i in 2..num {
//         mul = mul * i as u64;
//     }
//     mul
// }
//
// pub fn efficiency_rate(arrival_rate: f64, service_times: f64) -> f64 {
//     arrival_rate / (SERVER_COUNT as f64 * service_times)
// }
//
// /// Average number of customers in queue + currently in service
// pub fn average_customer_count_system(
//     arrival_rate: f64,
//     service_rate: f64,
//     service_times: Vec<f64>,
// ) -> f64 {
//     let efficiency_rate = efficiency_rate(arrival_rate, service_rate);
//     let avg_service = calc_avg_time(service_times);
//     let frac = (1.0 - (efficiency_rate / SERVER_COUNT as f64)).powi(2);
//     let expo = efficiency_rate.powi(SERVER_COUNT as i32);
//     let div = avg_service * factorial(SERVER_COUNT as u8) as f64 * SERVER_COUNT as f64 * frac;
//     ((expo / div) * probability_of_none(arrival_rate, 1.0) + (1.0 / avg_service)) * arrival_rate
// }

// Average number of customers in queue only
// pub fn average_customer_count_queue(arrival_rate: f64, service_rate: f64) -> f64 {
//     let efficiency_rate = efficiency_rate(arrival_rate, service_rate);
//     let expo =
//         efficiency_rate.powi(SERVER_COUNT as i32 + 1) * probability_of_none(arrival_rate, 1.0);
//     let frac = (1.0 - (efficiency_rate / SERVER_COUNT as f64)).powi(2);
//     let div = factorial(SERVER_COUNT as u8) as f64 * SERVER_COUNT as f64 * frac;
//     expo / div
// }

// pub fn average_customer_waiting_time_queue(arrival_rate: f64, service_rate: f64) -> f64 {
//     let efficiency_rate = efficiency_rate(arrival_rate, service_rate);
//     let top = efficiency_rate.powi(SERVER_COUNT as i32) * probability_of_none(arrival_rate, 1.0);
//     let frac = (1.0 - (efficiency_rate / SERVER_COUNT as f64)).powi(2);
//     let div = service_rate * factorial(SERVER_COUNT as u8) as f64 * SERVER_COUNT as f64 * frac;
//     top / div
// }

// pub fn average_customer_waiting_time_system(arrival_rate: f64, service_rate: f64) -> f64 {
//     let efficiency_rate = efficiency_rate(arrival_rate, service_rate);
//     let top = efficiency_rate.powi(SERVER_COUNT as i32) * probability_of_none(arrival_rate, 1.0);
//     let frac = (1.0 - (efficiency_rate / SERVER_COUNT as f64)).powi(2);
//     let div = service_rate * factorial(SERVER_COUNT as u8) as f64 * SERVER_COUNT as f64 * frac;
//     top / div + (1.0 / service_rate)
// }

pub fn get_all_service_times() -> (Vec<[f64; 4]>, Vec<u8>) {
    let transactions = list_transactions().unwrap();
    let mut server_1: Vec<f64> = Vec::new();
    let mut server_2: Vec<f64> = Vec::new();
    let mut server_3: Vec<f64> = Vec::new();
    let mut server_4: Vec<f64> = Vec::new();
    let mut service_times: Vec<[f64; SERVER_COUNT]> = Vec::new();
    let mut best_queue: Vec<u8> = Vec::new();
    for transaction in transactions {
        let teller_station = find_teller(transaction.server_id).expect("Unable to find teller");
        match teller_station.server_station {
            1 => server_1.push(transaction.duration as f64),
            2 => server_2.push(transaction.duration as f64),
            3 => server_3.push(transaction.duration as f64),
            4 => server_4.push(transaction.duration as f64),
            _ => {}
        };
    }

    for data in 0..server_1.len() {
        service_times.push([
            server_1[data],
            server_2[data],
            server_3[data],
            server_4[data],
        ])
    }

    for ser_time in &service_times {
        best_queue.push(calc_best_avg(*ser_time))
    }
    (service_times, best_queue)
}
#[derive(Debug)]
pub struct QueueStruct {
    pub queue: Vec<UserQueuePos>,
}

impl Default for QueueStruct {
    fn default() -> Self {
        QueueStruct {
            queue: Vec::with_capacity(CUSTOMER_COUNT),
        }
    }
}

impl QueueStruct {
    // pub fn new() -> Self {
    //     QueueStruct {
    //         queue: Vec::with_capacity(CUSTOMER_COUNT),
    //     }
    // }

    /*Main Queue Events*/
    pub fn add_item<'a>(
        &'a mut self,
        item: UserQuery,
        teller_queue: &MutexGuard<TellersQueue>,
        servers_queues: &'a mut Servers,
    ) -> Result<UserQueuePos, &str> {
        // let queue = self;
        if self.queue.len() < CUSTOMER_COUNT {
            match Self::assign_users(item.clone(), self.queue.len() + 1, servers_queues) {
                // the user position, position in server, assigned teller
                Ok((teller_loc, server_queue_pos)) => {
                    let user_pos = self.queue.len() + 1;
                    let teller_id = teller_queue.find_teller(teller_loc);
                    let timer = show_user_waiting_time(teller_id.server_id, self, server_queue_pos.clone());

                    let user_pos = UserQueuePos {
                        national_id: item.national_id,
                        queue_pos: user_pos,
                        teller_queue_pos: Some(server_queue_pos),
                        assigned_teller: Some(teller_loc),
                        timer,
                    };
                    self.queue.push(user_pos.clone());
                    Ok(user_pos)
                }
                Err(err) => Err(err),
            }
        } else {
            Err("Queue Length exceeds Maximum")
        }
    }
    pub fn remove_item(
        &mut self,
        user_queue_pos: usize,
        servers: &mut Servers,
    ) -> Result<(), &str> {
        let queue = &mut self.queue;
        if user_queue_pos < CUSTOMER_COUNT {
            let removed_user = queue.remove(user_queue_pos);
            match Self::remove_teller_users(removed_user, servers) {
                Ok(_) => Ok(()),
                Err(data) => Err(data),
            }
        } else {
            Err("User Doesn't Exist in the main Queue")
        }
    }

    /*Timer Events*/
    pub fn set_up_timer(
        &mut self,
        prev_remaining_time: f64,
        service_period: f64,
        server_index: usize,
    ) -> f64 {
        let mut timer: f64 = 0.0;
        if server_index > 2 {
            timer = (service_period * server_index as f64) + prev_remaining_time;
        } else if server_index <= 2 {
            timer = prev_remaining_time;
        }
        timer
    }
    pub fn get_waiting_time(
        &mut self,
        service_time: f64,
        prev_remaining_time: f64,
        user_server_pos: usize,
    ) -> f64 {
        self.set_up_timer(prev_remaining_time, service_time, user_server_pos)
    }

    /*Live Changes*/
    pub fn queue_change(&mut self, servers: &mut Servers) -> Result<(), &str> {
        for (index, user) in self.queue.iter_mut().enumerate() {
            user.queue_pos = index;
            let user_query = UserQuery {
                national_id: user.national_id.clone(),
            };
            match Self::assign_users(user_query, index, servers) {
                Ok(_) => {}
                Err(data) => return Err(data),
            }
        }
        Ok(())
    }

    /*Setting up user to teller in queue*/
    fn assign_users(
        user: UserQuery,
        index: usize,
        servers: &mut Servers,
    ) -> Result<(usize, usize), &'static str> {
        let pos: usize = index % SERVER_COUNT;
        match servers.add_server_customer(pos, user) {
            Ok(detail) => Ok(detail),
            Err(_) => Err("Unable to assign user to teller"),
        }
    }
    pub fn remove_teller_users(
        user: UserQueuePos,
        servers: &mut Servers,
    ) -> Result<&mut Servers, &'static str> {
        let pos: usize = user.queue_pos % SERVER_COUNT;
        match servers.remove_server_customer(user, pos) {
            Ok(data) => Ok(data),
            Err(_) => Err("Unable to remove user to teller"),
        }
    }
}
