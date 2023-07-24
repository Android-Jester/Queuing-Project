use std::cmp::min;
use crate::prelude::*;

fn calc_best_avg(avg_times: [f64; SERVER_COUNT]) -> u8 {
    let mut best = avg_times[0];
    for i in avg_times.iter() {
        best = best.max(*i)
    }
    let pos = avg_times.iter().position(|data| *data == best);
    pos.unwrap() as u8
}

pub fn get_all_service_times() -> (Vec<[f64; 4]>, Vec<u8>) {
    let transactions = list_transactions().unwrap();
    let mut servers: Vec<Vec<f64>> = vec![vec![], vec![], vec![], vec![]];
    let mut service_times: Vec<[f64; SERVER_COUNT]> = Vec::new();
    let mut best_queue: Vec<u8> = Vec::new();
    for transaction in transactions {
        let teller = find_teller(transaction.server_id);
        match teller {
            Ok(teller_data) => servers[teller_data.station as usize].push(transaction.duration as f64),
            Err(err) => {
                error!("ERROR: {err}");
            }
        }
    }
    let mut least_count = 0;

        for serve_data in servers.clone() {
            least_count = min(serve_data.len(), least_count);
        }
    for data in 0..=least_count {
        service_times.push([
            servers[0][data],
            servers[1][data],
            servers[2][data],
            servers[3][data],
        ])
    }

    for ser_time in &service_times {
        best_queue.push(calc_best_avg(*ser_time))
    }
    (service_times, best_queue)
}
