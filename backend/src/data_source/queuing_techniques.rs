use crate::data::models::{Teller, UserQueuePos};
use crate::data::{CUSTOMER_COUNT, SERVER_COUNT};
use crate::data_source::db_actions::{find_teller, list_transactions};
use crate::Servers;
use std::collections::VecDeque;
use std::f64::consts::E;
use crate::data::schema::Tellers::service_time;

// ///Calculate best of service_times
pub fn calc_best_avg(avg_times: [f64; SERVER_COUNT]) -> u8 {
    let mut best = avg_times[0];
    for i in avg_times.iter() {
        best = best.max(*i)
    }
    let pos = avg_times.iter().position(|data| *data == best).unwrap();
    pos as u8
}

fn calc_avg_time(service_times: Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for time in &service_times {
        sum = sum + time;
    }
    let res = sum / service_times.len() as f64;
    res
}

fn probability_of_none(arrival_rate: f64, time_interval: f64) -> f64 {
    E.powf(-(arrival_rate * time_interval))
}

fn factorial(num: u8) -> u64 {
    let mut mul: u64 = 1;
    for i in 2..num {
        mul = mul * i as u64;
    }
    mul
}

pub fn efficiciency_rate(arrival_rate: f64, service_times: f64) -> f64 {
    arrival_rate / (SERVER_COUNT as f64 * service_times)
}

/// Average number of customers in queue + currently in service
pub fn average_customer_count_system(
    arrival_rate: f64,
    service_rate: f64,
    service_times: Vec<f64>,
) -> f64 {
    let efficiency_rate = efficiciency_rate(arrival_rate, service_rate);
    let avg_service = calc_avg_time(service_times);
    let frac = (1.0 - (efficiency_rate / SERVER_COUNT as f64)).powi(2);
    let expo = efficiency_rate.powi(SERVER_COUNT as i32);
    let div = avg_service * factorial(SERVER_COUNT as u8) as f64 * SERVER_COUNT as f64 * frac;
    ((expo / div) * probability_of_none(arrival_rate, 1.0) + (1.0 / avg_service)) * arrival_rate
}

/// Average number of customers in queue only
pub fn average_customer_count_queue(arrival_rate: f64, service_rate: f64) -> f64 {
    let efficiency_rate = efficiciency_rate(arrival_rate, service_rate);
    let expo =
        efficiency_rate.powi(SERVER_COUNT as i32 + 1) * probability_of_none(arrival_rate, 1.0);
    let frac = (1.0 - (efficiency_rate / SERVER_COUNT as f64)).powi(2);
    let div = factorial(SERVER_COUNT as u8) as f64 * SERVER_COUNT as f64 * frac;
    expo / div
}

pub fn average_customer_waiting_time_queue(arrival_rate: f64, service_rate: f64) -> f64 {
    let efficiency_rate = efficiciency_rate(arrival_rate, service_rate);
    let top = efficiency_rate.powi(SERVER_COUNT as i32) * probability_of_none(arrival_rate, 1.0);
    let frac = (1.0 - (efficiency_rate / SERVER_COUNT as f64)).powi(2);
    let div = service_rate * factorial(SERVER_COUNT as u8) as f64 * SERVER_COUNT as f64 * frac;
    top / div
}

pub fn average_customer_waiting_time_system(arrival_rate: f64, service_rate: f64) -> f64 {
    let efficiency_rate = efficiciency_rate(arrival_rate, service_rate);
    let top = efficiency_rate.powi(SERVER_COUNT as i32) * probability_of_none(arrival_rate, 1.0);
    let frac = (1.0 - (efficiency_rate / SERVER_COUNT as f64)).powi(2);
    let div = service_rate * factorial(SERVER_COUNT as u8) as f64 * SERVER_COUNT as f64 * frac;
    top / div + (1.0 / service_rate)
}

pub fn get_all_service_times() -> (Vec<[f64; 4]>, Vec<u8>) {
    let transactions = list_transactions().unwrap();
    let mut server_1: Vec<f64> = Vec::new();
    let mut server_2: Vec<f64> = Vec::new();
    let mut server_3: Vec<f64> = Vec::new();
    let mut server_4: Vec<f64> = Vec::new();
    let mut service_times: Vec<[f64; SERVER_COUNT]> = Vec::new();
    let mut besto: Vec<u8> = Vec::new();
    for transaction in transactions {
        let teller_station = find_teller(transaction.server_id).expect("Unable to find teller");
        match teller_station {
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
        besto.push(calc_best_avg(*ser_time))
    }
    (service_times, besto)
}

pub struct QueueStruct {
    queue: VecDeque<UserQueuePos>,
}

impl QueueStruct {
    pub fn new() -> Self {
        QueueStruct {
            queue: VecDeque::with_capacity(CUSTOMER_COUNT),
        }
    }
    pub fn add_item(
        &mut self,
        item: UserQueuePos,
        servers_queues: &mut Servers,
    ) -> Result<&mut Self, &str> {
        self.queue.push_front(item.clone());
        Self::assign_users(self.queue.clone(), item.queue_pos, servers_queues);
        Ok(self)
    }

    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    pub fn remove_item(&mut self, main_queue_index: &usize, servers: &mut Servers) -> Result<&mut Self, &str> {
        let teller_pos: usize = main_queue_index % SERVER_COUNT;

        Self::remove_teller_users(self.queue.clone(), *index, servers);
        if self.queue_len() <= CUSTOMER_COUNT && self.queue_len() > 0 {
            match self.queue.remove(index.clone()) {
                None => Err("Unable to remove self"),
                Some(_) => {

                    Ok(self)
                }
            }
        } else {
            Err("Queue is full")
        }
    }
    pub fn set_up_timer(&mut self, prev_remaining_time: f64, service_period: f64, server_index: usize) -> f64 {
        let mut timer = 0.0;
        if server_index > 2 {
            timer = (service_period * server_index as f64) + prev_remaining_time as f64 ;
        } else if server_index <= 2 {
            timer = prev_remaining_time as f64;
        }
        timer
    }
    pub fn get_waiting_time(&mut self, teller: Teller, prev_remaining_time: f64, user_server_pos: usize) -> f64 {
        let timer = self.set_up_timer(prev_remaining_time, teller.service_time as f64, user_server_pos);
        timer
    }
    pub fn assign_users(
        self_queue: VecDeque<UserQueuePos>,
        main_queue_index: usize,
        servers: &mut Servers,
    ) -> Result<&mut Servers, &str> {
        let pos: usize = main_queue_index % SERVER_COUNT;
        let user_data = self_queue[main_queue_index].clone();
        match servers.add_server_customer(pos, user_data) {
            Ok(data ) => Ok(data),
            Err(d) => Err("Unable to assign user to teller")
        }
    }

    pub fn remove_teller_users(
        self_queue: VecDeque<UserQueuePos>,
        main_queue_index: usize,
        servers: &mut Servers,
    ) -> Result<&mut Servers, &str> {
        let pos: usize = main_queue_index % SERVER_COUNT;
        let user_data = self_queue[main_queue_index].clone();
        match servers.remove_server_customer(user_data, pos) {
            Ok(data) => Ok(data),
            Err(d) => Err("Unable to remove user to teller")
        }
    }
}
